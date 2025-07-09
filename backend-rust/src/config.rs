use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub stripe_secret_key: String,
    pub stripe_publishable_key: String,
    pub mt5_login: Option<String>,
    pub mt5_password: Option<String>,
    pub mt5_server: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_user: Option<String>,
    pub smtp_password: Option<String>,
    pub model_path: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            server_address: env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8000".to_string()),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET_KEY")
                .expect("JWT_SECRET_KEY must be set"),
            stripe_secret_key: env::var("STRIPE_SECRET_KEY")
                .expect("STRIPE_SECRET_KEY must be set"),
            stripe_publishable_key: env::var("STRIPE_PUBLISHABLE_KEY")
                .expect("STRIPE_PUBLISHABLE_KEY must be set"),
            mt5_login: env::var("MT5_LOGIN").ok(),
            mt5_password: env::var("MT5_PASSWORD").ok(),
            mt5_server: env::var("MT5_SERVER").ok(),
            smtp_host: env::var("SMTP_HOST").ok(),
            smtp_user: env::var("SMTP_USER").ok(),
            smtp_password: env::var("SMTP_PASSWORD").ok(),
            model_path: env::var("MODEL_PATH")
                .unwrap_or_else(|_| "../model/trading_model.onnx".to_string()),
        })
    }
}
