use crate::config::AppConfig;
use crate::supabase::auth::{sign_in, sign_out, sign_up, SupabaseAuth};
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct SupabaseClient {
    pub auth: SupabaseAuth,
    pub http: Client,
    pub config: Arc<AppConfig>,
}

impl SupabaseClient {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            auth: SupabaseAuth::new(&config),
            http: Client::new(),
            config,
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<serde_json::Value, crate::errors::AppError> {
        sign_up(
            &self.http,
            self.auth.gotrue_url(),
            self.auth.anon_key(),
            email,
            password,
        ).await
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<serde_json::Value, crate::errors::AppError> {
        sign_in(
            &self.http,
            self.auth.gotrue_url(),
            self.auth.anon_key(),
            email,
            password,
        ).await
    }

    pub async fn sign_out(&self, token: &str) -> Result<(), crate::errors::AppError> {
        sign_out(
            &self.http,
            self.auth.gotrue_url(),
            self.auth.anon_key(),
            token,
        ).await
    }

    pub async fn validate_token(&self, token: &str) -> Result<crate::supabase::auth::Claims, crate::errors::AppError> {
        self.auth.validate_token(&self.http, token).await
    }
}