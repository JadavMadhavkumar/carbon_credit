use sqlx::PgPool;
use std::sync::Arc;
use crate::config::AppConfig;
use crate::supabase::client::SupabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub config: AppConfig,
    pub supabase: SupabaseClient,
}

impl AppState {
    pub fn new(db: Option<PgPool>, config: AppConfig) -> Self {
        let supabase = SupabaseClient::new(Arc::new(config.clone()));
        Self { db, config, supabase }
    }
}

pub type SharedState = Arc<AppState>;

pub fn create_state(db: Option<PgPool>, config: AppConfig) -> SharedState {
    Arc::new(AppState::new(db, config))
}