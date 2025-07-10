from pydantic import BaseModel
from datetime import datetime
from typing import Optional

class StepRecordCreate(BaseModel):
    steps: int

class StepRecordResponse(BaseModel):
    id: int
    steps: int
    distance_km: float
    calories_burned: float
    created_at: datetime
    
    class Config:
        from_attributes = True

class WeightGoalResponse(BaseModel):
    id: int
    target_weight_loss_kg: float
    total_calories_needed: float
    calories_burned_so_far: float
    progress_percentage: float
    remaining_calories: float
    created_at: datetime
    
    class Config:
        from_attributes = True

class DashboardResponse(BaseModel):
    total_steps: int
    total_distance_km: float
    total_calories_burned: float
    weight_goal: WeightGoalResponse
    recent_records: list[StepRecordResponse]
