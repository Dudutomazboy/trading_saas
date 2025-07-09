use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use bigdecimal::BigDecimal;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub user_id: Uuid,
    pub robot_id: Uuid,
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub status: String,
    pub profit_loss: Option<f64>,
    pub commission: Option<f64>,
    pub swap: Option<f64>,
    pub ai_confidence: Option<f64>,
    pub ai_reasoning: Option<String>,
    pub broker_trade_id: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTradeRequest {
    pub robot_id: Uuid,
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub entry_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub ai_confidence: Option<f64>,
    pub ai_reasoning: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponse {
    pub id: Uuid,
    pub robot_id: Uuid,
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub status: String,
    pub profit_loss: Option<f64>,
    pub commission: Option<f64>,
    pub swap: Option<f64>,
    pub ai_confidence: Option<f64>,
    pub ai_reasoning: Option<String>,
    pub broker_trade_id: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Trade {
    pub fn new(
        user_id: Uuid,
        robot_id: Uuid,
        symbol: String,
        trade_type: String,
        volume: f64,
        entry_price: f64,
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
        ai_confidence: Option<f64>,
        ai_reasoning: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Trade {
            id: Uuid::new_v4(),
            user_id,
            robot_id,
            symbol,
            trade_type,
            volume,
            entry_price,
            exit_price: None,
            stop_loss,
            take_profit,
            status: "open".to_string(),
            profit_loss: None,
            commission: None,
            swap: None,
            ai_confidence,
            ai_reasoning,
            broker_trade_id: None,
            opened_at: now,
            closed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateTradeRequest,
    ) -> Result<Trade, sqlx::Error> {
        let trade = Trade::new(
            user_id,
            request.robot_id,
            request.symbol,
            request.trade_type,
            request.volume,
            request.entry_price,
            request.stop_loss,
            request.take_profit,
            request.ai_confidence,
            request.ai_reasoning,
        );

        sqlx::query!(
            r#"
            INSERT INTO trades (id, user_id, robot_id, symbol, trade_type, volume, entry_price, exit_price, stop_loss, take_profit, status, profit_loss, commission, swap, ai_confidence, ai_reasoning, broker_trade_id, opened_at, closed_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            "#,
            trade.id,
            trade.user_id,
            trade.robot_id,
            trade.symbol,
            trade.trade_type,
            trade.volume,
            trade.entry_price,
            trade.exit_price,
            trade.stop_loss,
            trade.take_profit,
            trade.status,
            trade.profit_loss,
            trade.commission,
            trade.swap,
            trade.ai_confidence,
            trade.ai_reasoning,
            trade.broker_trade_id,
            trade.opened_at,
            trade.closed_at,
            trade.created_at,
            trade.updated_at
        )
        .execute(pool)
        .await?;

        Ok(trade)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Trade>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, robot_id, symbol, trade_type, volume::FLOAT8 as volume, entry_price::FLOAT8 as entry_price, exit_price::FLOAT8 as exit_price, stop_loss::FLOAT8 as stop_loss, take_profit::FLOAT8 as take_profit, status, profit_loss::FLOAT8 as profit_loss, commission::FLOAT8 as commission, swap::FLOAT8 as swap, ai_confidence::FLOAT8 as ai_confidence, ai_reasoning, broker_trade_id, opened_at, closed_at, created_at, updated_at FROM trades WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let trades = rows.into_iter().map(|row| Trade {
            id: row.id,
            user_id: row.user_id,
            robot_id: row.robot_id,
            symbol: row.symbol,
            trade_type: row.trade_type,
            volume: row.volume,
            entry_price: row.entry_price,
            exit_price: row.exit_price,
            stop_loss: row.stop_loss,
            take_profit: row.take_profit,
            status: row.status,
            profit_loss: row.profit_loss,
            commission: if row.commission == 0.0 { None } else { Some(row.commission) },
            swap: if row.swap == 0.0 { None } else { Some(row.swap) },
            ai_confidence: if row.ai_confidence == 0.0 { None } else { Some(row.ai_confidence) },
            ai_reasoning: row.ai_reasoning.filter(|s| !s.is_empty()),
            broker_trade_id: row.broker_trade_id,
            opened_at: row.opened_at,
            closed_at: row.closed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(trades)
    }

    pub async fn find_by_robot_id(pool: &PgPool, robot_id: Uuid, user_id: Uuid) -> Result<Vec<Trade>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, robot_id, symbol, trade_type, volume::FLOAT8 as volume, entry_price::FLOAT8 as entry_price, exit_price::FLOAT8 as exit_price, stop_loss::FLOAT8 as stop_loss, take_profit::FLOAT8 as take_profit, status, profit_loss::FLOAT8 as profit_loss, commission::FLOAT8 as commission, swap::FLOAT8 as swap, ai_confidence::FLOAT8 as ai_confidence, ai_reasoning, broker_trade_id, opened_at, closed_at, created_at, updated_at FROM trades WHERE robot_id = $1 AND user_id = $2 ORDER BY created_at DESC"#,
            robot_id,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let trades = rows.into_iter().map(|row| Trade {
            id: row.id,
            user_id: row.user_id,
            robot_id: row.robot_id,
            symbol: row.symbol,
            trade_type: row.trade_type,
            volume: row.volume,
            entry_price: row.entry_price,
            exit_price: row.exit_price,
            stop_loss: row.stop_loss,
            take_profit: row.take_profit,
            status: row.status,
            profit_loss: row.profit_loss,
            commission: if row.commission == 0.0 { None } else { Some(row.commission) },
            swap: if row.swap == 0.0 { None } else { Some(row.swap) },
            ai_confidence: if row.ai_confidence == 0.0 { None } else { Some(row.ai_confidence) },
            ai_reasoning: row.ai_reasoning.filter(|s| !s.is_empty()),
            broker_trade_id: row.broker_trade_id,
            opened_at: row.opened_at,
            closed_at: row.closed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(trades)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<Trade>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, robot_id, symbol, trade_type, volume::FLOAT8 as volume, entry_price::FLOAT8 as entry_price, exit_price::FLOAT8 as exit_price, stop_loss::FLOAT8 as stop_loss, take_profit::FLOAT8 as take_profit, status, profit_loss::FLOAT8 as profit_loss, commission::FLOAT8 as commission, swap::FLOAT8 as swap, ai_confidence::FLOAT8 as ai_confidence, ai_reasoning, broker_trade_id, opened_at, closed_at, created_at, updated_at FROM trades WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Trade {
                id: row.id,
                user_id: row.user_id,
                robot_id: row.robot_id,
                symbol: row.symbol,
                trade_type: row.trade_type,
                volume: row.volume,
                entry_price: row.entry_price,
                exit_price: row.exit_price,
                stop_loss: row.stop_loss,
                take_profit: row.take_profit,
                status: row.status,
                profit_loss: row.profit_loss,
                commission: if row.commission == 0.0 { None } else { Some(row.commission) },
                swap: if row.swap == 0.0 { None } else { Some(row.swap) },
                ai_confidence: if row.ai_confidence == 0.0 { None } else { Some(row.ai_confidence) },
                ai_reasoning: row.ai_reasoning.filter(|s| !s.is_empty()),
                broker_trade_id: row.broker_trade_id,
                opened_at: row.opened_at,
                closed_at: row.closed_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn close_trade(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        exit_price: f64,
        commission: Option<f64>,
        swap: Option<f64>,
        broker_trade_id: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE trades SET exit_price = $1, status = 'closed', commission = $2, swap = $3, broker_trade_id = $4, closed_at = $5, updated_at = $6 WHERE id = $7 AND user_id = $8",
            exit_price,
            commission,
            swap,
            broker_trade_id,
            Utc::now(),
            Utc::now(),
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_open_trades(pool: &PgPool, user_id: Uuid) -> Result<Vec<Trade>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, robot_id, symbol, trade_type, volume::FLOAT8 as volume, entry_price::FLOAT8 as entry_price, exit_price::FLOAT8 as exit_price, stop_loss::FLOAT8 as stop_loss, take_profit::FLOAT8 as take_profit, status, profit_loss::FLOAT8 as profit_loss, commission::FLOAT8 as commission, swap::FLOAT8 as swap, ai_confidence::FLOAT8 as ai_confidence, ai_reasoning, broker_trade_id, opened_at, closed_at, created_at, updated_at FROM trades WHERE user_id = $1 AND status = 'open' ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let trades = rows.into_iter().map(|row| Trade {
            id: row.id,
            user_id: row.user_id,
            robot_id: row.robot_id,
            symbol: row.symbol,
            trade_type: row.trade_type,
            volume: row.volume,
            entry_price: row.entry_price,
            exit_price: row.exit_price,
            stop_loss: row.stop_loss,
            take_profit: row.take_profit,
            status: row.status,
            profit_loss: row.profit_loss,
            commission: if row.commission == 0.0 { None } else { Some(row.commission) },
            swap: if row.swap == 0.0 { None } else { Some(row.swap) },
            ai_confidence: if row.ai_confidence == 0.0 { None } else { Some(row.ai_confidence) },
            ai_reasoning: row.ai_reasoning.filter(|s| !s.is_empty()),
            broker_trade_id: row.broker_trade_id,
            opened_at: row.opened_at,
            closed_at: row.closed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(trades)
    }

    pub fn calculate_profit_loss(&self, current_price: f64) -> f64 {
        match self.trade_type.as_str() {
            "buy" => current_price - self.entry_price,
            "sell" => self.entry_price - current_price,
            _ => 0.0,
        }
    }

    pub fn is_profitable(&self, current_price: f64) -> bool {
        self.calculate_profit_loss(current_price) > 0.0
    }

    pub async fn get_statistics(pool: &PgPool, user_id: Uuid) -> Result<TradeStatistics, sqlx::Error> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_trades,
                COUNT(CASE WHEN profit_loss::FLOAT8 > 0 THEN 1 END) as winning_trades,
                COALESCE(SUM(profit_loss::FLOAT8), 0) as total_profit,
                COALESCE(AVG(profit_loss::FLOAT8), 0) as avg_profit
            FROM trades 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(TradeStatistics {
            total_trades: stats.total_trades.unwrap_or(0) as i32,
            winning_trades: stats.winning_trades.unwrap_or(0) as i32,
            total_profit: stats.total_profit.unwrap_or(0.0),
            avg_profit: stats.avg_profit.unwrap_or(0.0),
            win_rate: if stats.total_trades.unwrap_or(0) > 0 {
                (stats.winning_trades.unwrap_or(0) as f64 / stats.total_trades.unwrap_or(1) as f64) * 100.0
            } else {
                0.0
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeStatistics {
    pub total_trades: i32,
    pub winning_trades: i32,
    pub total_profit: f64,
    pub avg_profit: f64,
    pub win_rate: f64,
}

impl From<Trade> for TradeResponse {
    fn from(trade: Trade) -> Self {
        TradeResponse {
            id: trade.id,
            robot_id: trade.robot_id,
            symbol: trade.symbol,
            trade_type: trade.trade_type,
            volume: trade.volume,
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            stop_loss: trade.stop_loss,
            take_profit: trade.take_profit,
            status: trade.status,
            profit_loss: trade.profit_loss,
            commission: trade.commission,
            swap: trade.swap,
            ai_confidence: trade.ai_confidence,
            ai_reasoning: trade.ai_reasoning,
            broker_trade_id: trade.broker_trade_id,
            opened_at: trade.opened_at,
            closed_at: trade.closed_at,
            created_at: trade.created_at,
        }
    }
}
