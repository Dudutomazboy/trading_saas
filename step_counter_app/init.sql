-- Initialize the step_counter database
CREATE DATABASE IF NOT EXISTS step_counter;

-- Connect to the step_counter database
\c step_counter;

-- Create tables (these will also be created by SQLAlchemy, but this ensures they exist)
CREATE TABLE IF NOT EXISTS step_records (
    id SERIAL PRIMARY KEY,
    steps INTEGER NOT NULL,
    distance_km REAL NOT NULL,
    calories_burned REAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS weight_goals (
    id SERIAL PRIMARY KEY,
    target_weight_loss_kg REAL DEFAULT 20.0,
    total_calories_needed REAL DEFAULT 140000.0,
    calories_burned_so_far REAL DEFAULT 0.0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial weight goal if it doesn't exist
INSERT INTO weight_goals (target_weight_loss_kg, total_calories_needed, calories_burned_so_far)
SELECT 20.0, 140000.0, 0.0
WHERE NOT EXISTS (SELECT 1 FROM weight_goals);
