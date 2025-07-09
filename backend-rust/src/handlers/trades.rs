use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;

use crate::{
    models::{User, Trade, TradeResponse, TradeStatistics},
    errors::Result,
    AppState,
};

#[derive(Deserialize)]
pub struct ListTradesQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_trades(
    State(state): State<AppState>,
    Query(query): Query<ListTradesQuery>,
    current_user: User,
) -> Result<Json<Vec<TradeResponse>>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let trades = Trade::find_by_user_id(state.db.pool(), current_user.id).await?;
    let responses: Vec<TradeResponse> = trades.into_iter().map(|t| t.into()).collect();
    
    Ok(Json(responses))
}

pub async fn get_statistics(
    State(state): State<AppState>,
    current_user: User,
) -> Result<Json<TradeStatistics>> {
    let stats = Trade::get_statistics(state.db.pool(), current_user.id).await?;
    Ok(Json(stats))
}
