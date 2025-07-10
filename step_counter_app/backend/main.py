from fastapi import FastAPI, Depends, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy.orm import Session
from sqlalchemy import func
from database import get_db, create_tables, StepRecord, WeightGoal
from models import StepRecordCreate, StepRecordResponse, WeightGoalResponse, DashboardResponse
import uvicorn

app = FastAPI(title="Step Counter API", version="1.0.0")

# Enable CORS for React frontend
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000", "http://127.0.0.1:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

def calculate_distance_and_calories(steps: int):
    """
    Calculate distance and calories based on steps
    Assumptions:
    - Average step length: 0.762 meters (2.5 feet)
    - Walking speed: 5 km/h
    - Calories burned: 700 calories per hour at 5 km/h
    """
    # Distance calculation
    step_length_m = 0.762
    distance_m = steps * step_length_m
    distance_km = distance_m / 1000
    
    # Time calculation (hours)
    walking_speed_kmh = 5.0
    time_hours = distance_km / walking_speed_kmh
    
    # Calories calculation
    calories_per_hour = 700
    calories_burned = time_hours * calories_per_hour
    
    return distance_km, calories_burned

@app.on_event("startup")
async def startup_event():
    create_tables()

@app.get("/")
async def root():
    return {"message": "Step Counter API is running!"}

@app.post("/steps", response_model=StepRecordResponse)
async def add_steps(step_data: StepRecordCreate, db: Session = Depends(get_db)):
    """Add a new step record"""
    distance_km, calories_burned = calculate_distance_and_calories(step_data.steps)
    
    # Create step record
    db_step = StepRecord(
        steps=step_data.steps,
        distance_km=distance_km,
        calories_burned=calories_burned
    )
    db.add(db_step)
    
    # Update weight goal progress
    weight_goal = db.query(WeightGoal).first()
    if not weight_goal:
        weight_goal = WeightGoal()
        db.add(weight_goal)
    
    weight_goal.calories_burned_so_far += calories_burned
    
    db.commit()
    db.refresh(db_step)
    
    return db_step

@app.get("/steps", response_model=list[StepRecordResponse])
async def get_steps(limit: int = 10, db: Session = Depends(get_db)):
    """Get recent step records"""
    steps = db.query(StepRecord).order_by(StepRecord.created_at.desc()).limit(limit).all()
    return steps

@app.get("/dashboard", response_model=DashboardResponse)
async def get_dashboard(db: Session = Depends(get_db)):
    """Get dashboard data with totals and progress"""
    
    # Get totals
    totals = db.query(
        func.sum(StepRecord.steps).label('total_steps'),
        func.sum(StepRecord.distance_km).label('total_distance'),
        func.sum(StepRecord.calories_burned).label('total_calories')
    ).first()
    
    total_steps = totals.total_steps or 0
    total_distance_km = totals.total_distance or 0.0
    total_calories_burned = totals.total_calories or 0.0
    
    # Get or create weight goal
    weight_goal = db.query(WeightGoal).first()
    if not weight_goal:
        weight_goal = WeightGoal(calories_burned_so_far=total_calories_burned)
        db.add(weight_goal)
        db.commit()
        db.refresh(weight_goal)
    else:
        # Update calories burned so far
        weight_goal.calories_burned_so_far = total_calories_burned
        db.commit()
    
    # Calculate progress
    progress_percentage = min((weight_goal.calories_burned_so_far / weight_goal.total_calories_needed) * 100, 100)
    remaining_calories = max(weight_goal.total_calories_needed - weight_goal.calories_burned_so_far, 0)
    
    weight_goal_response = WeightGoalResponse(
        id=weight_goal.id,
        target_weight_loss_kg=weight_goal.target_weight_loss_kg,
        total_calories_needed=weight_goal.total_calories_needed,
        calories_burned_so_far=weight_goal.calories_burned_so_far,
        progress_percentage=progress_percentage,
        remaining_calories=remaining_calories,
        created_at=weight_goal.created_at
    )
    
    # Get recent records
    recent_records = db.query(StepRecord).order_by(StepRecord.created_at.desc()).limit(5).all()
    
    return DashboardResponse(
        total_steps=total_steps,
        total_distance_km=total_distance_km,
        total_calories_burned=total_calories_burned,
        weight_goal=weight_goal_response,
        recent_records=recent_records
    )

@app.delete("/steps/{step_id}")
async def delete_step_record(step_id: int, db: Session = Depends(get_db)):
    """Delete a step record"""
    step_record = db.query(StepRecord).filter(StepRecord.id == step_id).first()
    if not step_record:
        raise HTTPException(status_code=404, detail="Step record not found")
    
    db.delete(step_record)
    db.commit()
    return {"message": "Step record deleted successfully"}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
