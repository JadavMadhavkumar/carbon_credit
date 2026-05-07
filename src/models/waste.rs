use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WasteCategory {
    #[serde(rename = "organic")]
    Organic,
    #[serde(rename = "recyclables")]
    Recyclables,
    #[serde(rename = "ewaste")]
    EWaste,
    #[serde(rename = "textiles")]
    Textiles,
    #[serde(rename = "hazardous")]
    Hazardous,
    #[serde(rename = "bulky")]
    Bulky,
}

impl std::fmt::Display for WasteCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasteCategory::Organic => write!(f, "organic"),
            WasteCategory::Recyclables => write!(f, "recyclables"),
            WasteCategory::EWaste => write!(f, "ewaste"),
            WasteCategory::Textiles => write!(f, "textiles"),
            WasteCategory::Hazardous => write!(f, "hazardous"),
            WasteCategory::Bulky => write!(f, "bulky"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationMode {
    #[serde(rename = "self")]
    SelfReported,
    #[serde(rename = "ai")]
    AI,
    #[serde(rename = "facility")]
    Facility,
}

impl Default for VerificationMode {
    fn default() -> Self {
        VerificationMode::SelfReported
    }
}

impl VerificationMode {
    pub fn score(&self) -> f64 {
        match self {
            VerificationMode::SelfReported => 0.50,
            VerificationMode::AI => 0.85,
            VerificationMode::Facility => 1.00,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubmissionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verifying")]
    Verifying,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "anchored")]
    Anchored,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for SubmissionStatus {
    fn default() -> Self {
        SubmissionStatus::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityFactor {
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "lightly_contaminated")]
    LightlyContaminated,
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "severely_contaminated")]
    SeverelyContaminated,
}

impl Default for QualityFactor {
    fn default() -> Self {
        QualityFactor::Clean
    }
}

impl QualityFactor {
    pub fn value(&self) -> f64 {
        match self {
            QualityFactor::Clean => 1.00,
            QualityFactor::LightlyContaminated => 0.92,
            QualityFactor::Mixed => 0.80,
            QualityFactor::SeverelyContaminated => 0.60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteSubmission {
    pub id: Uuid,
    pub household_id: Uuid,
    pub submitted_at: DateTime<Utc>,
    pub waste_category: WasteCategory,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub verification_mode: VerificationMode,
    pub verification_score: f64,
    pub quality_factor: f64,
    pub raw_credits_kg: Option<f64>,
    pub blockchain_tx_hash: Option<String>,
    pub status: SubmissionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitWasteRequest {
    pub household_id: Option<Uuid>,
    pub waste_category: String,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub verification_mode: VerificationMode,
    pub quality_factor: Option<QualityFactor>,
    pub state_code: String,
}

impl SubmitWasteRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.weight_kg <= 0.0 {
            return Err(AppError::Validation("Weight must be positive".to_string()));
        }
        if self.weight_kg > 1000.0 {
            return Err(AppError::Validation("Weight cannot exceed 1000 kg".to_string()));
        }
        if self.waste_subtype.is_empty() {
            return Err(AppError::Validation("Waste subtype is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteSubmissionResponse {
    pub submission_id: Uuid,
    pub estimated_credits_kg: f64,
    pub credits_tonnes: f64,
    pub verification_score: f64,
    pub status: SubmissionStatus,
    pub breakdown: CarbonBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonBreakdown {
    pub waste_weight_kg: f64,
    pub emission_factor: f64,
    pub credit_multiplier: f64,
    pub locality_factor: f64,
    pub quality_factor: f64,
    pub base_credits_kg: f64,
    pub final_credits_kg: f64,
}