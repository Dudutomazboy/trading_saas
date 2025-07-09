use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::Response,
    body::Body,
};

use crate::{
    models::User,
    services::auth_service::AuthService,
    errors::AppError,
    AppState,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from Authorization header
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        })
        .ok_or_else(|| AppError::Auth("Missing authorization header".to_string()))?;

    // Verify token and extract user ID
    let user_id = AuthService::extract_user_id_from_token(token, &state.config.jwt_secret)?;

    // Fetch user from database
    let user = User::find_by_id(state.db.pool(), user_id)
        .await?
        .ok_or_else(|| AppError::Auth("User not found".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Auth("Account is disabled".to_string()));
    }

    // Add user to request extensions
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

pub async fn admin_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Get user from request extensions (should be set by auth_middleware)
    let user = request
        .extensions()
        .get::<User>()
        .ok_or_else(|| AppError::Auth("Authentication required".to_string()))?;

    // Check if user is admin
    if !user.is_superuser {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    Ok(next.run(request).await)
}

// Extractor for getting the current user from request
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<User>()
            .cloned()
            .ok_or_else(|| AppError::Auth("Authentication required".to_string()))
    }
}
