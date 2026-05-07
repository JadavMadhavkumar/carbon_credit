mod carbon_engine;
mod config;
mod db;
mod errors;
mod models;
mod state;
mod supabase;

use uuid::Uuid;
use sqlx::Row;
use carbon_engine::{
    CarbonCalculator, ProcessingMethod, calculate_credits,
    CalculateEmissionRequest, CalculateEmissionResponse, 
    CalculateCarbonCreditRequest, CalculateCarbonCreditResponse,
};
use carbon_engine::biochar::BiocharProcessor;
use crate::models::carbon::BiocharProcessRequest;
use config::AppConfig;
use db::migrations;
use errors::AppError;
use models::waste::{VerificationMode, QualityFactor, SubmitWasteRequest};
use state::SharedState;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};

// API handlers would go here if we had them

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,plastic_waste_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Falkon Carbon Platform v2.0");

    let config = AppConfig::load()?;
    tracing::info!("Server config: {}", config.server_address());

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Database connected");

    sqlx::query(migrations::CREATE_USERS).execute(&db).await?;
    sqlx::query(migrations::CREATE_HOUSEHOLDS).execute(&db).await?;
    sqlx::query(migrations::CREATE_WASTE_SUBMISSIONS).execute(&db).await?;
    sqlx::query(migrations::CREATE_CREDIT_LEDGER).execute(&db).await?;
    sqlx::query(migrations::CREATE_BIOCHAR_RECORDS).execute(&db).await?;
    sqlx::query(migrations::CREATE_REPORTS).execute(&db).await?;
    sqlx::query(migrations::CREATE_MUNICIPALITIES).execute(&db).await?;
    sqlx::query(migrations::CREATE_INDEXES).execute(&db).await?;
    tracing::info!("Migrations done");

    let state = state::create_state(db, config.clone());
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/carbon/emission", post(calculate_emission_handler))
        .route("/api/v1/carbon/credit", post(calculate_credit_handler))
        .route("/api/v1/waste/submit", post(submit_waste_handler))
        .route("/api/v1/biochar/process", post(process_biochar_handler))
        .route("/api/v1/analytics", get(analytics_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "Falkon Carbon API v2.0 - OK"
}

async fn calculate_emission_handler(
    State(_state): State<SharedState>,
    Json(req): Json<CalculateEmissionRequest>,
) -> Result<Json<CalculateEmissionResponse>, AppError> {
    let result = CarbonCalculator::calculate_emission(req)?;
    Ok(Json(result))
}

async fn calculate_credit_handler(
    State(_state): State<SharedState>,
    Json(req): Json<CalculateCarbonCreditRequest>,
) -> Result<Json<CalculateCarbonCreditResponse>, AppError> {
    let result = CarbonCalculator::calculate_credit(req)?;
    Ok(Json(result))
}

async fn submit_waste_handler(
    State(state): State<SharedState>,
    Json(req): Json<SubmitWasteRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate()?;

    let category = req.waste_category.to_lowercase();
    let subtype = req.waste_subtype.to_lowercase();
    let state_code = req.state_code.to_uppercase();

    let quality_factor = req.quality_factor.unwrap_or_default();
    let processing_method = ProcessingMethod::Recycling;

    let verification_mode = req.verification_mode.clone();
    let result = calculate_credits(
        &category,
        &subtype,
        req.weight_kg,
        verification_mode,
        Some(quality_factor.clone()),
        &state_code,
        Some(models::carbon::BaselineScenario::Landfill),
        processing_method,
    )?;

    let submission_id = uuid::Uuid::now_v7();
    let verification_score = req.verification_mode.score();

    sqlx::query(
        "INSERT INTO waste_submissions (id, household_id, waste_category, waste_subtype, weight_kg, verification_mode, verification_score, quality_factor, raw_credits_kg, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'pending')"
    )
    .bind(submission_id)
    .bind(req.household_id.unwrap_or(uuid::Uuid::nil()))
    .bind(&category)
    .bind(&subtype)
    .bind(req.weight_kg)
    .bind(serde_json::to_string(&req.verification_mode).unwrap_or_default())
    .bind(verification_score)
    .bind(quality_factor.value())
    .bind(result.credits_kg)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "submission_id": submission_id,
        "estimated_credits_kg": result.credits_kg,
        "credits_tonnes": result.credits_tonnes,
        "verification_score": verification_score,
        "status": "pending",
    })))
}

async fn process_biochar_handler(
    State(_state): State<SharedState>,
    Json(req): Json<BiocharProcessRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = BiocharProcessor::process(req)?;
    Ok(Json(serde_json::json!({
        "biochar_output_kg": result.biochar_output_kg,
        "carbon_sequestered_kg": result.carbon_sequestered_kg,
        "carbon_credits_kg": result.carbon_credits_kg,
        "token_units": result.token_units,
    })))
}

async fn analytics_handler(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let row = sqlx::query(
        "SELECT COALESCE(SUM(raw_credits_kg), 0) as total_credits, COUNT(*) as total_submissions FROM waste_submissions WHERE status IN ('verified', 'anchored')"
    )
    .fetch_one(&state.db)
    .await?;

    let total_credits: f64 = row.get("total_credits");
    let total_submissions: i64 = row.get("total_submissions");

    Ok(Json(serde_json::json!({
        "total_credits_kg": total_credits,
        "total_credits_tonnes": total_credits / 1000.0,
        "total_submissions": total_submissions,
    })))
}