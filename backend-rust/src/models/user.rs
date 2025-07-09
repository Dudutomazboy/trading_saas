use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub subscription_plan: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub subscription_plan: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = Utc::now();
        User {
            id: Uuid::new_v4(),
            email,
            password_hash,
            is_active: true,
            is_superuser: false,
            subscription_plan: "free".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn create(
        pool: &PgPool,
        request: CreateUserRequest,
    ) -> Result<User, sqlx::Error> {
        // Simple password hashing - in production use bcrypt
        let password_hash = request.password; // This should be hashed
        let user = User::new(request.email, password_hash);

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, is_active, is_superuser, subscription_plan, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.is_active,
            user.is_superuser,
            user.subscription_plan,
            user.created_at,
            user.updated_at
        )
        .execute(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, is_active, is_superuser, subscription_plan, created_at, updated_at FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, is_active, is_superuser, subscription_plan, created_at, updated_at FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        // Simple password verification - in production use bcrypt
        // For now, just compare directly (this should be hashed comparison)
        self.password_hash == password
    }

    pub async fn update_last_login(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET updated_at = NOW() WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list_all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, is_active, is_superuser, subscription_plan, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn update_subscription_plan(
        pool: &PgPool,
        id: Uuid,
        plan: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET subscription_plan = $1, updated_at = $2 WHERE id = $3",
            plan,
            Utc::now(),
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            is_active: user.is_active,
            is_superuser: user.is_superuser,
            subscription_plan: user.subscription_plan,
            created_at: user.created_at,
        }
    }
}
