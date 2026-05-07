use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub log_level: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_service_key: Option<String>,
    #[cfg(feature = "runtime-tokio")]
    pub redis_url: Option<String>,
    #[cfg(feature = "runtime-tokio")]
    pub kafka_broker: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, anyhow::Error> {
        dotenvy::dotenv().ok();

        let supabase_url = env::var("SUPABASE_URL")
            .unwrap_or_else(|_| "https://ntsvtmvbkcmvyllizxrf.supabase.co".to_string());

        let config = Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| format!(
                    "postgresql://postgres:[PASSWORD]@db.ntsvtmvbkcmvyllizxrf.supabase.co:5432/postgres"
                )),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "falkon-carbon-secret-key-change-in-production".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            supabase_url,
            supabase_anon_key: env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "".to_string()),
            supabase_service_key: env::var("SUPABASE_SERVICE_KEY").ok(),
            #[cfg(feature = "runtime-tokio")]
            redis_url: env::var("REDIS_URL").ok(),
            #[cfg(feature = "runtime-tokio")]
            kafka_broker: env::var("KAFKA_BROKER").ok(),
        };

        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn gotrue_url(&self) -> String {
        format!("{}/auth/v1", self.supabase_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_defaults() {
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("JWT_SECRET");
        let config = AppConfig::load().unwrap();
        assert_eq!(config.server_port, 3000);
    }
}