pub mod factors;
pub mod calculator;
pub mod biochar;

use crate::models;
use serde::{Deserialize, Serialize};

pub use calculator::CarbonCalculator;
pub use models::carbon::{CalculateEmissionRequest, CalculateEmissionResponse, CalculateCarbonCreditRequest, CalculateCarbonCreditResponse, CarbonCreditBreakdown};
pub use factors::{get_emission_factor, get_locality_factor, get_grid_emission_factor, get_processing_factor};

pub fn calculate_credits(
    category: &str,
    subtype: &str,
    weight_kg: f64,
    verification_mode: crate::models::waste::VerificationMode,
    quality_factor: Option<crate::models::waste::QualityFactor>,
    state_code: &str,
    baseline_scenario: Option<crate::models::carbon::BaselineScenario>,
    processing_method: ProcessingMethod,
) -> Result<calculator::CalculationResult, crate::errors::AppError> {
    let req = CalculateCarbonCreditRequest {
        waste_category: category.to_string(),
        waste_subtype: subtype.to_string(),
        weight_kg,
        processing_method,
        verification_mode,
        quality_factor,
        state_code: state_code.to_string(),
        baseline_scenario,
    };

    let response = CarbonCalculator::calculate_credit(req)?;
    Ok(calculator::CalculationResult {
        credits_kg: response.credits_kg,
        credits_tonnes: response.credits_tonnes,
        breakdown: response.breakdown,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessingMethod {
    #[serde(rename = "landfill")]
    Landfill,
    #[serde(rename = "composting")]
    Composting,
    #[serde(rename = "biogas")]
    Biogas,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "energy_recovery")]
    EnergyRecovery,
    #[serde(rename = "biochar")]
    Biochar,
}