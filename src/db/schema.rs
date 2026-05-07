use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Household {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: Option<String>,
    pub state_code: String,
    pub address: serde_json::Value,
    pub municipality_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasteSubmission {
    pub id: Uuid,
    pub household_id: Uuid,
    pub submitted_at: DateTime<Utc>,
    pub waste_category: String,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub verification_mode: String,
    pub verification_score: f64,
    pub quality_factor: f64,
    pub raw_credits_kg: Option<f64>,
    pub processing_method: String,
    pub blockchain_tx_hash: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditLedger {
    pub id: Uuid,
    pub household_id: Uuid,
    pub submission_id: Option<Uuid>,
    pub credits_kg: f64,
    pub token_id: Option<String>,
    pub ledger_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiocharRecord {
    pub id: Uuid,
    pub household_id: Uuid,
    pub biomass_type: String,
    pub biomass_input_kg: f64,
    pub biochar_output_kg: f64,
    pub carbon_sequestered_kg: f64,
    pub carbon_credits_kg: f64,
    pub pyrolysis_temp_c: f64,
    pub residence_time_min: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub household_id: Option<Uuid>,
    pub municipality_id: Option<Uuid>,
    pub report_type: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_credits_kg: f64,
    pub total_submissions: i64,
    pub category_breakdown: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Municipality {
    pub id: Uuid,
    pub name: String,
    pub state_code: String,
    pub total_households: i64,
    pub created_at: DateTime<Utc>,
}