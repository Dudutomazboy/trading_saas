use axum::{
    extract::State,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::User,
    services::auth_service::AuthService,
    errors::Result,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleLoginRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub subscription_plan: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            is_active: user.is_active,
            is_superuser: user.is_superuser,
            subscription_plan: user.subscription_plan,
            created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>> {
    // Check if user already exists
    if let Some(_) = User::find_by_email(state.db.pool(), &payload.email).await? {
        return Err(crate::errors::AppError::Validation("Email already exists".to_string()));
    }

    // Create user
    let create_request = crate::models::user::CreateUserRequest {
        email: payload.email,
        password: payload.password,
    };
    let user = User::create(state.db.pool(), create_request).await?;

    // Generate token
    let token = AuthService::create_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": UserResponse::from(user)
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>> {
    // Find user by email
    let user = User::find_by_email(state.db.pool(), &payload.email)
        .await?
        .ok_or_else(|| crate::errors::AppError::Auth("Invalid credentials".to_string()))?;

    // Verify password
    if !user.verify_password(&payload.password) {
        return Err(crate::errors::AppError::Auth("Invalid credentials".to_string()));
    }

    // Check if user is active
    if !user.is_active {
        return Err(crate::errors::AppError::Auth("Account is disabled".to_string()));
    }

    // Update last login
    User::update_last_login(state.db.pool(), user.id).await?;

    // Generate token
    let token = AuthService::create_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": UserResponse::from(user)
    })))
}

pub async fn google_login(
    State(state): State<AppState>,
    Json(payload): Json<GoogleLoginRequest>,
) -> Result<Json<serde_json::Value>> {
    // Verify Google token and get user info
    let google_user = AuthService::verify_google_token(&payload.token).await?;
    
    // Check if user exists
    let user = match User::find_by_email(state.db.pool(), &google_user.email).await? {
        Some(existing_user) => {
            // Update last login
            User::update_last_login(state.db.pool(), existing_user.id).await?;
            existing_user
        }
        None => {
            // Create new user from Google info
            let create_request = crate::models::user::CreateUserRequest {
                email: google_user.email,
                password: uuid::Uuid::new_v4().to_string(), // Random password for OAuth users
            };
            User::create(state.db.pool(), create_request).await?
        }
    };

    // Check if user is active
    if !user.is_active {
        return Err(crate::errors::AppError::Auth("Account is disabled".to_string()));
    }

    // Generate token
    let token = AuthService::create_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": UserResponse::from(user)
    })))
}

pub async fn me(
    State(_state): State<AppState>,
    current_user: User,
) -> Result<Json<UserResponse>> {
    Ok(Json(UserResponse::from(current_user)))
}
