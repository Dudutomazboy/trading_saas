use axum::{
    extract::State,
    response::Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{User, Trade, TradingRobot, TradeStatistics},
    errors::Result,
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub user_info: DashboardUserInfo,
    pub trading_stats: TradeStatistics,
    pub active_robots: Vec<DashboardRobot>,
    pub recent_trades: Vec<DashboardTrade>,
    pub performance_summary: PerformanceSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUserInfo {
    pub email: String,
    pub subscription_plan: String,
    pub account_balance: f64,
    pub total_robots: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardRobot {
    pub id: uuid::Uuid,
    pub name: String,
    pub symbol: String,
    pub status: String,
    pub total_profit: f64,
    pub win_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardTrade {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub trade_type: String,
    pub profit_loss: Option<f64>,
    pub status: String,
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub today_profit: f64,
    pub week_profit: f64,
    pub month_profit: f64,
    pub total_profit: f64,
    pub best_performing_symbol: Option<String>,
    pub worst_performing_symbol: Option<String>,
}

pub async fn get_dashboard(
    State(state): State<AppState>,
    current_user: User,
) -> Result<Json<DashboardData>> {
    // Get trading statistics
    let trading_stats = Trade::get_statistics(state.db.pool(), current_user.id).await?;

    // Get active robots
    let robots = TradingRobot::find_by_user_id(state.db.pool(), current_user.id).await?;
    let active_robots: Vec<DashboardRobot> = robots
        .into_iter()
        .filter(|r| r.status == "active")
        .map(|r| {
            let status = r.status.clone();
            let win_rate = r.calculate_win_rate();
            DashboardRobot {
                id: r.id,
                name: r.name,
                symbol: "EURUSD".to_string(), // TODO: Get from robot config
                status,
                total_profit: 0.0, // TODO: Calculate from trades
                win_rate,
            }
        })
        .collect();

    // Get recent trades
    let trades = Trade::find_by_user_id(state.db.pool(), current_user.id).await?;
    let recent_trades: Vec<DashboardTrade> = trades
        .into_iter()
        .map(|t| DashboardTrade {
            id: t.id,
            symbol: t.symbol,
            trade_type: t.trade_type,
            profit_loss:t.profit_loss,
            status: t.status,
            opened_at: t.opened_at,
        })
        .collect();

    // Calculate performance summary
    let performance_summary = PerformanceSummary {
        today_profit: 0.0, // TODO: Calculate from trades
        week_profit: 0.0,  // TODO: Calculate from trades
        month_profit: 0.0, // TODO: Calculate from trades
        total_profit: trading_stats.total_profit,
        best_performing_symbol: None, // TODO: Calculate from trades
        worst_performing_symbol: None, // TODO: Calculate from trades
    };

    let dashboard_data = DashboardData {
        user_info: DashboardUserInfo {
            email: current_user.email,
            subscription_plan: current_user.subscription_plan,
            account_balance: 10000.0, // TODO: Get from broker connection
            total_robots: active_robots.len() as i32,
        },
        trading_stats,
        active_robots,
        recent_trades,
        performance_summary,
    };

    Ok(Json(dashboard_data))
}
