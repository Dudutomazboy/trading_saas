use axum::{
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod models;
mod handlers;
mod services;
mod app_middleware;
mod errors;

use config::Config;
use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok(); // no main.rs

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trading_saas_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(Config::from_env()?);
    
    // Initialize database
    let db = Database::new(&config.database_url).await?;
    
    // Run migrations
    db.migrate().await?;

    // Create application state
    let state = AppState {
        db,
        config: config.clone(),
    };

    // Build our application with routes
    let app = create_app(state);

    // Run the server
    let addr = config.server_address.parse::<std::net::SocketAddr>()?;
    tracing::info!("Server running on {}", config.server_address);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/google", post(handlers::auth::google_login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route("/api/v1/users", get(handlers::users::list_users))
        .route("/api/v1/users/:id", get(handlers::users::get_user))
        .route("/api/v1/subscriptions", get(handlers::subscriptions::list_subscriptions))
        .route("/api/v1/subscriptions", post(handlers::subscriptions::create_subscription))
        .route("/api/v1/brokers", get(handlers::brokers::list_brokers))
        .route("/api/v1/brokers", post(handlers::brokers::create_broker))
        .route("/api/v1/brokers/:id/test", post(handlers::brokers::test_connection))
        .route("/api/v1/robots", get(handlers::robots::list_robots))
        .route("/api/v1/robots", post(handlers::robots::create_robot))
        .route("/api/v1/robots/:id/start", post(handlers::robots::start_robot))
        .route("/api/v1/robots/:id/stop", post(handlers::robots::stop_robot))
        .route("/api/v1/trades", get(handlers::trades::list_trades))
        .route("/api/v1/trades/statistics", get(handlers::trades::get_statistics))
        .route("/api/v1/dashboard", get(handlers::dashboard::get_dashboard))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth_middleware));

    // Admin routes (admin authentication required)
    let admin_routes = Router::new()
        .route("/api/v1/admin/users", get(handlers::admin::list_all_users))
        .route("/api/v1/admin/stats", get(handlers::admin::get_system_stats))
        .layer(middleware::from_fn(app_middleware::admin_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth_middleware));

    // Combine all routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(state)
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
