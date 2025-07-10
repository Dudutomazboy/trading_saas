from sqlalchemy import create_engine, Column, Integer, Float, DateTime, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from datetime import datetime
import os

DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://postgres:password@localhost:5432/step_counter")

engine = create_engine(DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

class StepRecord(Base):
    __tablename__ = "step_records"
    
    id = Column(Integer, primary_key=True, index=True)
    steps = Column(Integer, nullable=False)
    distance_km = Column(Float, nullable=False)
    calories_burned = Column(Float, nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)

class WeightGoal(Base):
    __tablename__ = "weight_goals"
    
    id = Column(Integer, primary_key=True, index=True)
    target_weight_loss_kg = Column(Float, default=20.0)
    total_calories_needed = Column(Float, default=140000.0)  # 20kg * 7000 cal/kg
    calories_burned_so_far = Column(Float, default=0.0)
    created_at = Column(DateTime, default=datetime.utcnow)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

def create_tables():
    Base.metadata.create_all(bind=engine)
