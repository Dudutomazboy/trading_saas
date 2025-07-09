use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BrokerConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub broker_type: String,
    pub api_key: String, // Encrypted
    pub api_secret: String, // Encrypted
    pub server: Option<String>,
    pub login: Option<String>,
    pub is_active: bool,
    pub is_demo: bool,
    pub last_test_at: Option<DateTime<Utc>>,
    pub last_test_status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateBrokerConnectionRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub broker_type: String,
    pub api_key: String,
    pub api_secret: String,
    pub server: Option<String>,
    pub login: Option<String>,
    pub is_demo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokerConnectionResponse {
    pub id: Uuid,
    pub name: String,
    pub broker_type: String,
    pub server: Option<String>,
    pub login: Option<String>,
    pub is_active: bool,
    pub is_demo: bool,
    pub last_test_at: Option<DateTime<Utc>>,
    pub last_test_status: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResponse {
    pub success: bool,
    pub message: String,
    pub account_info: Option<AccountInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_number: String,
    pub balance: f64,
    pub equity: f64,
    pub margin: f64,
    pub free_margin: f64,
    pub currency: String,
}

impl BrokerConnection {
    pub fn new(
        user_id: Uuid,
        name: String,
        broker_type: String,
        api_key: String,
        api_secret: String,
        server: Option<String>,
        login: Option<String>,
        is_demo: bool,
    ) -> Self {
        let now = Utc::now();
        BrokerConnection {
            id: Uuid::new_v4(),
            user_id,
            name,
            broker_type,
            api_key, // Should be encrypted before storing
            api_secret, // Should be encrypted before storing
            server,
            login,
            is_active: true,
            is_demo,
            last_test_at: None,
            last_test_status: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateBrokerConnectionRequest,
    ) -> Result<BrokerConnection, sqlx::Error> {
        let broker_connection = BrokerConnection::new(
            user_id,
            request.name,
            request.broker_type,
            request.api_key, // TODO: Encrypt before storing
            request.api_secret, // TODO: Encrypt before storing
            request.server,
            request.login,
            request.is_demo,
        );

        sqlx::query!(
            r#"
            INSERT INTO broker_connections (id, user_id, name, broker_type, api_key, api_secret, server, login, is_active, is_demo, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            broker_connection.id,
            broker_connection.user_id,
            broker_connection.name,
            broker_connection.broker_type,
            broker_connection.api_key,
            broker_connection.api_secret,
            broker_connection.server,
            broker_connection.login,
            broker_connection.is_active,
            broker_connection.is_demo,
            broker_connection.created_at,
            broker_connection.updated_at
        )
        .execute(pool)
        .await?;

        Ok(broker_connection)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<BrokerConnection>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT id, user_id, name, broker_type, api_key, api_secret, server, login, is_active, is_demo, last_test_at, last_test_status, created_at, updated_at FROM broker_connections WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let connections = rows.into_iter().map(|row| BrokerConnection {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            broker_type: row.broker_type,
            api_key: row.api_key,
            api_secret: row.api_secret,
            server: row.server,
            login: row.login,
            is_active: row.is_active,
            is_demo: row.is_demo,
            last_test_at: row.last_test_at,
            last_test_status: row.last_test_status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(connections)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<BrokerConnection>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, name, broker_type, api_key, api_secret, server, login, is_active, is_demo, last_test_at, last_test_status, created_at, updated_at FROM broker_connections WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(BrokerConnection {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                broker_type: row.broker_type,
                api_key: row.api_key,
                api_secret: row.api_secret,
                server: row.server,
                login: row.login,
                is_active: row.is_active,
                is_demo: row.is_demo,
                last_test_at: row.last_test_at,
                last_test_status: row.last_test_status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_test_result(
        pool: &PgPool,
        id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE broker_connections SET last_test_at = $1, last_test_status = $2, updated_at = $3 WHERE id = $4",
            Utc::now(),
            status,
            Utc::now(),
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_active(
        pool: &PgPool,
        id: Uuid,
        is_active: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE broker_connections SET is_active = $1, updated_at = $2 WHERE id = $3",
            is_active,
            Utc::now(),
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl From<BrokerConnection> for BrokerConnectionResponse {
    fn from(connection: BrokerConnection) -> Self {
        BrokerConnectionResponse {
            id: connection.id,
            name: connection.name,
            broker_type: connection.broker_type,
            server: connection.server,
            login: connection.login,
            is_active: connection.is_active,
            is_demo: connection.is_demo,
            last_test_at: connection.last_test_at,
            last_test_status: connection.last_test_status,
            created_at: connection.created_at,
        }
    }
}
