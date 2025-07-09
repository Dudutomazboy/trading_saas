use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

pub struct AuthService;

impl AuthService {
    pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| AppError::Jwt(e))
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Jwt(e))
    }

    pub fn extract_user_id_from_token(token: &str, secret: &str) -> Result<Uuid, AppError> {
        let claims = Self::verify_token(token, secret)?;
        claims.sub.parse::<Uuid>()
            .map_err(|e| AppError::Auth(format!("Invalid user ID in token: {}", e)))
    }

    pub async fn verify_google_token(token: &str) -> Result<GoogleUser, AppError> {
        // For now, we'll create a mock implementation
        // In production, you would verify the token with Google's API
        // https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={token}
        
        // Mock implementation - replace with actual Google token verification
        if token.starts_with("mock_google_token_") {
            let email = token.replace("mock_google_token_", "");
            return Ok(GoogleUser {
                email: format!("{}@gmail.com", email),
                name: format!("User {}", email),
                picture: "https://via.placeholder.com/150".to_string(),
            });
        }

        // For production, implement actual Google token verification
        // This is a simplified version - you should use reqwest to call Google's API
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", token))
            .send()
            .await
            .map_err(|e| AppError::Auth(format!("Failed to verify Google token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Auth("Invalid Google token".to_string()));
        }

        let google_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Auth(format!("Failed to parse Google response: {}", e)))?;

        let email = google_response["email"]
            .as_str()
            .ok_or_else(|| AppError::Auth("No email in Google token".to_string()))?;

        let name = google_response["name"]
            .as_str()
            .unwrap_or("Unknown User");

        let picture = google_response["picture"]
            .as_str()
            .unwrap_or("");

        Ok(GoogleUser {
            email: email.to_string(),
            name: name.to_string(),
            picture: picture.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret";
        
        let token = AuthService::create_token(user_id, secret).unwrap();
        let claims = AuthService::verify_token(&token, secret).unwrap();
        let extracted_id = AuthService::extract_user_id_from_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(extracted_id, user_id);
    }

    #[tokio::test]
    async fn test_mock_google_token() {
        let token = "mock_google_token_testuser";
        let result = AuthService::verify_google_token(token).await.unwrap();
        
        assert_eq!(result.email, "testuser@gmail.com");
        assert_eq!(result.name, "User testuser");
    }
}
