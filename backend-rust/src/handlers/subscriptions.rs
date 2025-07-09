use axum::{
    extract::State,
    response::Json,
};

use crate::{
    models::{User, Subscription, CreateSubscriptionRequest, SubscriptionResponse},
    errors::Result,
    AppState,
};

pub async fn list_subscriptions(
    State(state): State<AppState>,
    current_user: User,
) -> Result<Json<Option<SubscriptionResponse>>> {
    let subscription = Subscription::find_by_user_id(state.db.pool(), current_user.id).await?;
    Ok(Json(subscription.map(|s| s.into())))
}

pub async fn create_subscription(
    State(state): State<AppState>,
    current_user: User,
    Json(payload): Json<CreateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>> {
    // TODO: Integrate with Stripe
    let subscription = Subscription::create(
        state.db.pool(),
        current_user.id,
        payload.plan_name,
        None,
        None,
    ).await?;

    Ok(Json(subscription.into()))
}
