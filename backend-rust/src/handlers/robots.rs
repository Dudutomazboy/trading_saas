use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{User, TradingRobot, CreateTradingRobotRequest, TradingRobotResponse},
    errors::{Result, AppError},
    AppState,
};

pub async fn list_robots(
    State(state): State<AppState>,
    current_user: User,
) -> Result<Json<Vec<TradingRobotResponse>>> {
    let robots = TradingRobot::find_by_user_id(state.db.pool(), current_user.id).await?;
    let responses: Vec<TradingRobotResponse> = robots.into_iter().map(|r| r.into()).collect();
    Ok(Json(responses))
}

pub async fn create_robot(
    State(state): State<AppState>,
    current_user: User,
    Json(payload): Json<CreateTradingRobotRequest>,
) -> Result<Json<TradingRobotResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let robot = TradingRobot::create(state.db.pool(), current_user.id, payload).await?;
    Ok(Json(robot.into()))
}

pub async fn start_robot(
    State(state): State<AppState>,
    Path(robot_id): Path<Uuid>,
    current_user: User,
) -> Result<Json<TradingRobotResponse>> {
    let robot = TradingRobot::find_by_id(state.db.pool(), robot_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Trading robot not found".to_string()))?;

    TradingRobot::update_status(state.db.pool(), robot_id, current_user.id, "active").await?;

    // TODO: Start the actual trading logic
    
    let updated_robot = TradingRobot::find_by_id(state.db.pool(), robot_id, current_user.id)
        .await?
        .unwrap();

    Ok(Json(updated_robot.into()))
}

pub async fn stop_robot(
    State(state): State<AppState>,
    Path(robot_id): Path<Uuid>,
    current_user: User,
) -> Result<Json<TradingRobotResponse>> {
    let robot = TradingRobot::find_by_id(state.db.pool(), robot_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Trading robot not found".to_string()))?;

    TradingRobot::update_status(state.db.pool(), robot_id, current_user.id, "stopped").await?;

    // TODO: Stop the actual trading logic
    
    let updated_robot = TradingRobot::find_by_id(state.db.pool(), robot_id, current_user.id)
        .await?
        .unwrap();

    Ok(Json(updated_robot.into()))
}
