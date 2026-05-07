use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub wallet_address: Option<String>,
    pub state_code: String,
    pub municipality_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Household {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: Option<String>,
    pub state_code: String,
    pub address: serde_json::Value,
    pub municipality_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub state_code: String,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if !self.email.contains('@') {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }
        if self.password.len() < 8 {
            return Err(AppError::Validation("Password must be at least 8 characters".to_string()));
        }
        if self.full_name.is_empty() {
            return Err(AppError::Validation("Full name is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub state_code: String,
    pub wallet_address: Option<String>,
    pub total_credits_kg: f64,
    pub total_submissions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, expires_in_seconds: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            sub: user_id,
            exp: now + expires_in_seconds,
            iat: now,
        }
    }
}