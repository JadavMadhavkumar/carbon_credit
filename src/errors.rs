use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Carbon calculation error: {0}")]
    CarbonCalculation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Invalid waste category: {0}")]
    InvalidCategory(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::CarbonCalculation(ref msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Internal(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::RateLimit(ref msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
            AppError::InvalidCategory(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

