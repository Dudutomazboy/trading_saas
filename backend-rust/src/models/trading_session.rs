use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use bigdecimal::{BigDecimal, FromPrimitive};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub robot_id: Uuid,
    pub status: String,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub total_profit: f64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTradingSessionRequest {
    pub robot_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingSessionResponse {
    pub id: Uuid,
    pub robot_id: Uuid,
    pub status: String,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub total_profit: f64,
    pub win_rate: f64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl TradingSession {
    pub fn new(user_id: Uuid, robot_id: Uuid) -> Self {
        let now = Utc::now();
        TradingSession {
            id: Uuid::new_v4(),
            user_id,
            robot_id,
            status: "active".to_string(),
            total_trades: 0,
            winning_trades: 0,
            total_profit: 0.0,
            started_at: now,
            ended_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateTradingSessionRequest,
    ) -> Result<TradingSession, sqlx::Error> {
        let session = TradingSession::new(user_id, request.robot_id);

        sqlx::query!(
            r#"
            INSERT INTO trading_sessions (id, user_id, robot_id, status, total_trades, winning_trades, total_profit, started_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            session.id,
            session.user_id,
            session.robot_id,
            session.status,
            session.total_trades,
            session.winning_trades,
            session.total_profit,
            session.started_at,
            session.created_at,
            session.updated_at
        )
        .execute(pool)
        .await?;

        Ok(session)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<TradingSession>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, robot_id, status, total_trades, winning_trades, total_profit::FLOAT8 as total_profit, started_at, ended_at, created_at, updated_at FROM trading_sessions WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let sessions = rows.into_iter().map(|row| TradingSession {
            id: row.id,
            user_id: row.user_id,
            robot_id: row.robot_id,
            status: row.status,
            total_trades: row.total_trades,
            winning_trades: row.winning_trades,
            total_profit: row.total_profit,
            started_at: row.started_at,
            ended_at: row.ended_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(sessions)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<TradingSession>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, robot_id, status, total_trades, winning_trades, total_profit::FLOAT8 as total_profit, started_at, ended_at, created_at, updated_at FROM trading_sessions WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(TradingSession {
                id: row.id,
                user_id: row.user_id,
                robot_id: row.robot_id,
                status: row.status,
                total_trades: row.total_trades,
                winning_trades: row.winning_trades,
                total_profit: row.total_profit,
                started_at: row.started_at,
                ended_at: row.ended_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn calculate_win_rate(&self) -> f64 {
        if self.total_trades == 0 {
            0.0
        } else {
            (self.winning_trades as f64 / self.total_trades as f64) * 100.0
        }
    }

    pub fn calculate_duration_minutes(&self) -> Option<i64> {
        if let Some(ended_at) = self.ended_at {
            Some((ended_at - self.started_at).num_minutes())
        } else {
            Some((Utc::now() - self.started_at).num_minutes())
        }
    }
}

impl From<TradingSession> for TradingSessionResponse {
    fn from(session: TradingSession) -> Self {
        let win_rate = session.calculate_win_rate();
        let duration_minutes = session.calculate_duration_minutes();
        
        TradingSessionResponse {
            id: session.id,
            robot_id: session.robot_id,
            status: session.status.clone(),
            total_trades: session.total_trades,
            winning_trades: session.winning_trades,
            total_profit: session.total_profit,
            win_rate,
            started_at: session.started_at,
            ended_at: session.ended_at,
            duration_minutes,
            created_at: session.created_at,
        }
    }
}
