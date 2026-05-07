use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonCredit {
    pub id: Uuid,
    pub household_id: Uuid,
    pub submission_id: Option<Uuid>,
    pub credits_kg: Decimal,
    pub token_id: Option<String>,
    pub ledger_type: LedgerType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LedgerType {
    #[serde(rename = "earn")]
    Earn,
    #[serde(rename = "redeem")]
    Redeem,
    #[serde(rename = "transfer")]
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBalance {
    pub household_id: Uuid,
    pub total_earned_kg: Decimal,
    pub total_redeemed_kg: Decimal,
    pub available_kg: Decimal,
    pub available_tonnes: Decimal,
    pub token_count: i64,
}

impl CreditBalance {
    pub fn calculate(earnings: Decimal, redemptions: Decimal) -> Self {
        let available = earnings - redemptions;
        Self {
            household_id: Uuid::nil(),
            total_earned_kg: earnings,
            total_redeemed_kg: redemptions,
            available_kg: available,
            available_tonnes: available / Decimal::from(1000),
            token_count: (available.to_string().parse::<f64>().unwrap_or(0.0) * 1000.0).ceil() as i64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateEmissionRequest {
    pub waste_category: String,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub processing_method: ProcessingMethod,
    pub distance_km: Option<f64>,
    pub energy_kwh: Option<f64>,
    pub state_code: String,
}

pub use crate::carbon_engine::ProcessingMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateEmissionResponse {
    pub total_emission_kg: f64,
    pub waste_emission_kg: f64,
    pub transport_emission_kg: f64,
    pub processing_emission_kg: f64,
    pub energy_emission_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateCarbonCreditRequest {
    pub waste_category: String,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub processing_method: ProcessingMethod,
    pub verification_mode: crate::models::waste::VerificationMode,
    pub quality_factor: Option<crate::models::waste::QualityFactor>,
    pub state_code: String,
    pub baseline_scenario: Option<BaselineScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BaselineScenario {
    #[serde(rename = "landfill")]
    Landfill,
    #[serde(rename = "incineration")]
    Incineration,
    #[serde(rename = "open_burning")]
    OpenBurning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateCarbonCreditResponse {
    pub credits_kg: f64,
    pub credits_tonnes: f64,
    pub baseline_emission_kg: f64,
    pub actual_emission_kg: f64,
    pub net_reduction_kg: f64,
    pub token_units: i64,
    pub breakdown: CarbonCreditBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonCreditBreakdown {
    pub waste_weight_kg: f64,
    pub emission_factor: f64,
    pub credit_multiplier: f64,
    pub verification_score: f64,
    pub locality_factor: f64,
    pub quality_factor: f64,
    pub additionality_factor: f64,
    pub leakage_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiocharProcessRequest {
    pub biomass_type: String,
    pub biomass_weight_kg: f64,
    pub pyrolysis_temperature_c: f64,
    pub residence_time_minutes: i32,
    pub state_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiocharProcessResponse {
    pub biochar_output_kg: f64,
    pub carbon_sequestered_kg: f64,
    pub carbon_credits_kg: f64,
    pub carbon_credits_tonnes: f64,
    pub token_units: i64,
    pub breakdown: BiocharBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiocharBreakdown {
    pub biomass_input_kg: f64,
    pub biochar_yield_rate: f64,
    pub carbon_content: f64,
    pub stability_factor: f64,
    pub carbon_retained_kg: f64,
    pub emission_avoided_kg: f64,
}