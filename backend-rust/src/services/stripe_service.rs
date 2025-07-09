use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCustomer {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub current_period_start: i64,
    pub current_period_end: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct StripeApiCustomer {
    id: String,
    email: String,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StripeApiSubscription {
    id: String,
    customer: String,
    status: String,
    current_period_start: i64,
    current_period_end: i64,
}

pub struct StripeService {
    secret_key: String,
    client: Client,
}

impl StripeService {
    pub fn new(secret_key: String) -> Self {
        StripeService {
            secret_key,
            client: Client::new(),
        }
    }

    pub async fn create_customer(&self, email: &str, name: Option<&str>) -> Result<StripeCustomer> {
        // For now, return mock data. In production, implement actual Stripe API calls
        if self.secret_key.starts_with("sk_test_mock") {
            return Ok(StripeCustomer {
                id: format!("cus_{}", uuid::Uuid::new_v4().to_string().replace("-", "")),
                email: email.to_string(),
                name: name.map(|s| s.to_string()),
            });
        }

        // Actual Stripe API implementation would go here
        let mut params = HashMap::new();
        params.insert("email", email);
        if let Some(name) = name {
            params.insert("name", name);
        }

        let response = self
            .client
            .post("https://api.stripe.com/v1/customers")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Stripe API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Stripe API error: {}",
                response.status()
            )));
        }

        let stripe_customer: StripeApiCustomer = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Stripe response: {}", e)))?;

        Ok(StripeCustomer {
            id: stripe_customer.id,
            email: stripe_customer.email,
            name: stripe_customer.name,
        })
    }

    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
        payment_method_id: &str,
    ) -> Result<StripeSubscription> {
        // For now, return mock data. In production, implement actual Stripe API calls
        if self.secret_key.starts_with("sk_test_mock") {
            return Ok(StripeSubscription {
                id: format!("sub_{}", uuid::Uuid::new_v4().to_string().replace("-", "")),
                customer_id: customer_id.to_string(),
                status: "active".to_string(),
                current_period_start: chrono::Utc::now().timestamp(),
                current_period_end: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp(),
            });
        }

        // Actual Stripe API implementation would go here
        let mut params = HashMap::new();
        params.insert("customer", customer_id);
        params.insert("items[0][price]", price_id);
        params.insert("default_payment_method", payment_method_id);

        let response = self
            .client
            .post("https://api.stripe.com/v1/subscriptions")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Stripe API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Stripe API error: {}",
                response.status()
            )));
        }

        let stripe_subscription: StripeApiSubscription = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Stripe response: {}", e)))?;

        Ok(StripeSubscription {
            id: stripe_subscription.id,
            customer_id: stripe_subscription.customer,
            status: stripe_subscription.status,
            current_period_start: stripe_subscription.current_period_start,
            current_period_end: stripe_subscription.current_period_end,
        })
    }

    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<()> {
        // For now, just log. In production, implement actual Stripe API calls
        if self.secret_key.starts_with("sk_test_mock") {
            tracing::info!("Mock: Cancelling Stripe subscription: {}", subscription_id);
            return Ok(());
        }

        // Actual Stripe API implementation would go here
        let response = self
            .client
            .delete(&format!(
                "https://api.stripe.com/v1/subscriptions/{}",
                subscription_id
            ))
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .send()
            .await
            .map_err(|e| AppError::External(format!("Stripe API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::External(format!(
                "Stripe API error: {}",
                response.status()
            )));
        }

        tracing::info!("Successfully cancelled Stripe subscription: {}", subscription_id);
        Ok(())
    }
}
