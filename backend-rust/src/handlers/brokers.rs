use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{User, BrokerConnection, CreateBrokerConnectionRequest, BrokerConnectionResponse, TestConnectionResponse},
    errors::{Result, AppError},
    AppState,
};

pub async fn list_brokers(
    State(state): State<AppState>,
    current_user: User,
) -> Result<Json<Vec<BrokerConnectionResponse>>> {
    let connections = BrokerConnection::find_by_user_id(state.db.pool(), current_user.id).await?;
    let responses: Vec<BrokerConnectionResponse> = connections.into_iter().map(|c| c.into()).collect();
    Ok(Json(responses))
}

pub async fn create_broker(
    State(state): State<AppState>,
    current_user: User,
    Json(payload): Json<CreateBrokerConnectionRequest>,
) -> Result<Json<BrokerConnectionResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let connection = BrokerConnection::create(state.db.pool(), current_user.id, payload).await?;
    Ok(Json(connection.into()))
}

pub async fn test_connection(
    State(state): State<AppState>,
    Path(connection_id): Path<Uuid>,
    current_user: User,
) -> Result<Json<TestConnectionResponse>> {
    let connection = BrokerConnection::find_by_id(state.db.pool(), connection_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Broker connection not found".to_string()))?;

    // TODO: Implement actual MT5 connection test
    let test_result = TestConnectionResponse {
        success: true,
        message: "Connection test successful".to_string(),
        account_info: None,
    };

    // Update test result in database
    BrokerConnection::update_test_result(
        state.db.pool(),
        connection_id,
        if test_result.success { "success" } else { "failed" },
    ).await?;

    Ok(Json(test_result))
}
