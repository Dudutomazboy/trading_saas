use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    models::User,
    errors::Result,
    AppState,
};

#[derive(Deserialize)]
pub struct AdminUsersQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_robots: i64,
    pub active_robots: i64,
    pub total_trades: i64,
    pub total_profit: f64,
    pub subscription_breakdown: SubscriptionBreakdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionBreakdown {
    pub free: i64,
    pub essential: i64,
    pub pro: i64,
    pub elite: i64,
}

pub async fn list_all_users(
    State(state): State<AppState>,
    Query(query): Query<AdminUsersQuery>,
    _current_user: User,
) -> Result<Json<Vec<UserResponse>>> {
    // This endpoint should be protected by admin middleware
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let users = User::list_all(state.db.pool(), limit, offset).await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

    Ok(Json(user_responses))
}

pub async fn get_system_stats(
    State(state): State<AppState>,
    _current_user: User,
) -> Result<Json<SystemStats>> {
    // This endpoint should be protected by admin middleware
    
    // Get user statistics
    let total_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let active_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE is_active = true"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    // Get robot statistics
    let total_robots = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM trading_robots"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let active_robots = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM trading_robots WHERE status = 'active'"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    // Get trade statistics
    let total_trades = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM trades"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let total_profit: f64 = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(profit_loss), 0.0)::FLOAT FROM trades WHERE status = 'closed'"
    )
    .fetch_one(state.db.pool())
    .await?
    .unwrap_or(0.0);

    // Get subscription breakdown
    let free_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE subscription_plan = 'free'"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let essential_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE subscription_plan = 'essential'"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let pro_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE subscription_plan = 'pro'"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let elite_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE subscription_plan = 'elite'"
    )
    .fetch_one(state.db.pool())
    .await?
    .map(|count| count as i64)
    .unwrap_or(0);

    let stats = SystemStats {
        total_users,
        active_users,
        total_robots,
        active_robots,
        total_trades,
        total_profit,
        subscription_breakdown: SubscriptionBreakdown {
            free: free_users,
            essential: essential_users,
            pro: pro_users,
            elite: elite_users,
        },
    };

    Ok(Json(stats))
}
