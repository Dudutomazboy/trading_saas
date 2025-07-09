use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_name: String,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub status: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub name: String,
    pub price: f64,
    pub currency: String,
    pub interval: String,
    pub max_robots: i32,
    pub max_assets: i32,
    pub max_operations_per_day: i32,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_name: String,
    pub payment_method_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub plan_name: String,
    pub status: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub plan_details: SubscriptionPlan,
}

impl Subscription {
    pub fn new(user_id: Uuid, plan_name: String) -> Self {
        let now = Utc::now();
        Subscription {
            id: Uuid::new_v4(),
            user_id,
            plan_name,
            stripe_subscription_id: None,
            stripe_customer_id: None,
            status: "active".to_string(),
            current_period_start: now,
            current_period_end: now + chrono::Duration::days(30),
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        plan_name: String,
        stripe_subscription_id: Option<String>,
        stripe_customer_id: Option<String>,
    ) -> Result<Subscription, sqlx::Error> {
        let subscription = Subscription {
            stripe_subscription_id,
            stripe_customer_id,
            ..Subscription::new(user_id, plan_name)
        };

        sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, user_id, plan_name, stripe_subscription_id, stripe_customer_id, status, current_period_start, current_period_end, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            subscription.id,
            subscription.user_id,
            subscription.plan_name,
            subscription.stripe_subscription_id,
            subscription.stripe_customer_id,
            subscription.status,
            subscription.current_period_start,
            subscription.current_period_end,
            subscription.created_at,
            subscription.updated_at
        )
        .execute(pool)
        .await?;

        Ok(subscription)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Subscription>, sqlx::Error> {
        let subscription = sqlx::query_as!(
            Subscription,
            r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", plan_name, stripe_subscription_id, stripe_customer_id, status, current_period_start as "current_period_start: DateTime<Utc>", current_period_end as "current_period_end: DateTime<Utc>", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM subscriptions WHERE user_id = $1 AND status = 'active' ORDER BY created_at DESC LIMIT 1"#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(subscription)
    }

    pub async fn update_status(
        pool: &PgPool,
        subscription_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE subscriptions SET status = $1, updated_at = $2 WHERE id = $3",
            status,
            Utc::now(),
            subscription_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn get_plan_details(&self) -> SubscriptionPlan {
        match self.plan_name.as_str() {
            "free" => SubscriptionPlan {
                name: "Free".to_string(),
                price: 0.0,
                currency: "USD".to_string(),
                interval: "month".to_string(),
                max_robots: 0,
                max_assets: 0,
                max_operations_per_day: 0,
                features: vec!["Demo trading".to_string(), "Community support".to_string()],
            },
            "essential" => SubscriptionPlan {
                name: "Essential".to_string(),
                price: 29.99,
                currency: "USD".to_string(),
                interval: "month".to_string(),
                max_robots: 1,
                max_assets: 1,
                max_operations_per_day: 50,
                features: vec![
                    "1 trading robot".to_string(),
                    "1 asset".to_string(),
                    "50 operations/day".to_string(),
                    "Email support".to_string(),
                ],
            },
            "pro" => SubscriptionPlan {
                name: "Pro".to_string(),
                price: 99.99,
                currency: "USD".to_string(),
                interval: "month".to_string(),
                max_robots: 5,
                max_assets: 10,
                max_operations_per_day: 200,
                features: vec![
                    "5 trading robots".to_string(),
                    "10 assets".to_string(),
                    "200 operations/day".to_string(),
                    "Priority support".to_string(),
                    "Advanced reports".to_string(),
                ],
            },
            "elite" => SubscriptionPlan {
                name: "Elite".to_string(),
                price: 299.99,
                currency: "USD".to_string(),
                interval: "month".to_string(),
                max_robots: -1, // Unlimited
                max_assets: -1, // Unlimited
                max_operations_per_day: -1, // Unlimited
                features: vec![
                    "Unlimited robots".to_string(),
                    "Unlimited assets".to_string(),
                    "Unlimited operations".to_string(),
                    "Dedicated support".to_string(),
                    "Real-time optimization".to_string(),
                    "Custom strategies".to_string(),
                ],
            },
            _ => SubscriptionPlan {
                name: "Unknown".to_string(),
                price: 0.0,
                currency: "USD".to_string(),
                interval: "month".to_string(),
                max_robots: 0,
                max_assets: 0,
                max_operations_per_day: 0,
                features: vec![],
            },
        }
    }
}

impl From<Subscription> for SubscriptionResponse {
    fn from(subscription: Subscription) -> Self {
        let plan_details = subscription.get_plan_details();
        SubscriptionResponse {
            id: subscription.id,
            plan_name: subscription.plan_name,
            status: subscription.status,
            current_period_start: subscription.current_period_start,
            current_period_end: subscription.current_period_end,
            plan_details,
        }
    }
}
