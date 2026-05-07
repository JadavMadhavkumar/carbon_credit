use serde::{Deserialize, Serialize};
use crate::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub exp: i64,
    pub iat: i64,
    pub iss: Option<String>,
    pub email_confirmed_at: Option<String>,
    pub app_metadata: Option<serde_json::Value>,
    pub user_metadata: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct SupabaseAuth {
    gotrue_url: String,
    anon_key: String,
}

impl SupabaseAuth {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            gotrue_url: config.gotrue_url(),
            anon_key: config.supabase_anon_key.clone(),
        }
    }

    pub fn gotrue_url(&self) -> &str {
        &self.gotrue_url
    }

    pub fn anon_key(&self) -> &str {
        &self.anon_key
    }

    pub async fn validate_token(&self, http: &reqwest::Client, token: &str) -> Result<Claims, crate::errors::AppError> {
        let response = http
            .get(format!("{}/user", self.gotrue_url))
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| crate::errors::AppError::Unauthorized(format!("Failed to validate token: {}", e)))?;

        if !response.status().is_success() {
            return Err(crate::errors::AppError::Unauthorized("Invalid or expired token".to_string()));
        }

        let user: serde_json::Value = response.json().await
            .map_err(|e| crate::errors::AppError::Internal(format!("Failed to parse user response: {}", e)))?;

        Ok(Claims {
            sub: user.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            email: user.get("email").and_then(|v| v.as_str()).map(String::from),
            role: user.get("role").and_then(|v| v.as_str()).map(String::from),
            exp: 0,
            iss: None,
            iat: 0,
            email_confirmed_at: user.get("email_confirmed_at").and_then(|v| v.as_str()).map(String::from),
            app_metadata: user.get("app_metadata").cloned(),
            user_metadata: user.get("user_metadata").cloned(),
        })
    }
}

pub async fn sign_up(
    client: &reqwest::Client,
    gotrue_url: &str,
    anon_key: &str,
    email: &str,
    password: &str,
) -> Result<serde_json::Value, crate::errors::AppError> {
    let response = client
        .post(format!("{}/signup", gotrue_url))
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Failed to sign up: {}", e)))?;

    let status = response.status();
    let body = response.json::<serde_json::Value>().await
        .map_err(|e| crate::errors::AppError::Internal(format!("Failed to parse response: {}", e)))?;

    if status.is_success() {
        Ok(body)
    } else {
        Err(crate::errors::AppError::Internal(
            body.get("msg").and_then(|v| v.as_str()).unwrap_or("Sign up failed").to_string()
        ))
    }
}

pub async fn sign_in(
    client: &reqwest::Client,
    gotrue_url: &str,
    anon_key: &str,
    email: &str,
    password: &str,
) -> Result<serde_json::Value, crate::errors::AppError> {
    let response = client
        .post(format!("{}/token?grant_type=password", gotrue_url))
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Failed to sign in: {}", e)))?;

    let status = response.status();
    let body = response.json::<serde_json::Value>().await
        .map_err(|e| crate::errors::AppError::Internal(format!("Failed to parse response: {}", e)))?;

    if status.is_success() {
        Ok(body)
    } else {
        Err(crate::errors::AppError::Internal(
            body.get("error_description").or_else(|| body.get("msg"))
                .and_then(|v| v.as_str())
                .unwrap_or("Sign in failed")
                .to_string()
        ))
    }
}

pub async fn sign_out(
    client: &reqwest::Client,
    gotrue_url: &str,
    anon_key: &str,
    token: &str,
) -> Result<(), crate::errors::AppError> {
    let response = client
        .post(format!("{}/logout", gotrue_url))
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Failed to sign out: {}", e)))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(crate::errors::AppError::Internal("Sign out failed".to_string()))
    }
}