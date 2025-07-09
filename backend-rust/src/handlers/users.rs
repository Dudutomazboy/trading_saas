use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{User, UserResponse},
    errors::Result,
    AppState,
};

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
    current_user: User,
) -> Result<Json<Vec<UserResponse>>> {
    // Only allow admins to list all users
    if !current_user.is_superuser {
        return Ok(Json(vec![current_user.into()]));
    }

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let users = User::list_all(state.db.pool(), limit, offset).await?;
    let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

    Ok(Json(user_responses))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    current_user: User,
) -> Result<Json<UserResponse>> {
    // Users can only access their own profile, unless they're admin
    if current_user.id != user_id && !current_user.is_superuser {
        return Err(crate::errors::AppError::Forbidden("Access denied".to_string()));
    }

    let user = User::find_by_id(state.db.pool(), user_id)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}
