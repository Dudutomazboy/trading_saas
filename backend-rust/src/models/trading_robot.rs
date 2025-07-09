use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingRobot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub strategy: String,
    pub status: String,
    pub risk_config: serde_json::Value,
    pub performance_metrics: serde_json::Value,
    pub last_signal_at: Option<DateTime<Utc>>,
    pub total_trades: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTradingRobotRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub strategy: String,
    pub risk_config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingRobotResponse {
    pub id: Uuid,
    pub name: String,
    pub strategy: String,
    pub status: String,
    pub risk_config: serde_json::Value,
    pub performance_metrics: serde_json::Value,
    pub last_signal_at: Option<DateTime<Utc>>,
    pub total_trades: i32,
    pub total_profit: f64,
    pub winning_trades: i32,
    pub win_rate: f64,
    pub created_at: DateTime<Utc>,
}

impl TradingRobot {
    pub fn new(
        user_id: Uuid,
        name: String,
        strategy: String,
    ) -> Self {
        let now = Utc::now();
        TradingRobot {
            id: Uuid::new_v4(),
            user_id,
            name,
            strategy,
            status: "inactive".to_string(),
            risk_config: serde_json::json!({
                "max_risk_per_trade": 0.02,
                "stop_loss_pips": 20,
                "take_profit_pips": 40,
                "max_daily_loss": 0.05
            }),
            performance_metrics: serde_json::json!({
                "total_profit": 0.0,
                "winning_trades": 0
            }),
            last_signal_at: None,
            total_trades: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateTradingRobotRequest,
    ) -> Result<TradingRobot, sqlx::Error> {
        let robot = TradingRobot::new(
            user_id,
            request.name,
            request.strategy,
        );

        sqlx::query!(
            r#"
            INSERT INTO trading_robots (id, user_id, name, strategy, status, risk_config, performance_metrics, last_signal_at, total_trades, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            robot.id,
            robot.user_id,
            robot.name,
            robot.strategy,
            robot.status,
            robot.risk_config,
            robot.performance_metrics,
            robot.last_signal_at,
            robot.total_trades,
            robot.created_at,
            robot.updated_at
        )
        .execute(pool)
        .await?;

        Ok(robot)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<TradingRobot>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, name, strategy, status, risk_config, performance_metrics, last_signal_at, total_trades, created_at, updated_at FROM trading_robots WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let robots = rows.into_iter().map(|row| TradingRobot {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            strategy: row.strategy.unwrap_or_default(),
            status: row.status,
            risk_config: row.risk_config,
            performance_metrics: row.performance_metrics.unwrap_or_default(),
            last_signal_at: row.last_signal_at,
            total_trades: row.total_trades,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(robots)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<TradingRobot>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, name, strategy, status, risk_config, performance_metrics, last_signal_at, total_trades, created_at, updated_at FROM trading_robots WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(TradingRobot {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                strategy: row.strategy.unwrap_or_default(),
                status: row.status,
                risk_config: row.risk_config,
                performance_metrics: row.performance_metrics.unwrap_or_default(),
                last_signal_at: row.last_signal_at,
                total_trades: row.total_trades,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE trading_robots SET status = $1, updated_at = $2 WHERE id = $3 AND user_id = $4",
            status,
            Utc::now(),
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn get_total_profit(&self) -> f64 {
        self.performance_metrics
            .get("total_profit")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    pub fn get_winning_trades(&self) -> i32 {
        self.performance_metrics
            .get("winning_trades")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32
    }

    pub fn calculate_win_rate(&self) -> f64 {
        if self.total_trades == 0 {
            0.0
        } else {
            (self.get_winning_trades() as f64 / self.total_trades as f64) * 100.0
        }
    }
}

impl From<TradingRobot> for TradingRobotResponse {
    fn from(robot: TradingRobot) -> Self {
        let total_profit = robot.get_total_profit();
        let winning_trades = robot.get_winning_trades();
        let win_rate = robot.calculate_win_rate();
        
        TradingRobotResponse {
            id: robot.id,
            name: robot.name,
            strategy: robot.strategy,
            status: robot.status,
            risk_config: robot.risk_config,
            performance_metrics: robot.performance_metrics,
            last_signal_at: robot.last_signal_at,
            total_trades: robot.total_trades,
            total_profit,
            winning_trades,
            win_rate,
            created_at: robot.created_at,
        }
    }
}
