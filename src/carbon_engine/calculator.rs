use crate::carbon_engine::factors::{get_emission_factor, get_locality_factor, get_grid_emission_factor, get_processing_factor};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use crate::models::carbon::{BaselineScenario, CalculateCarbonCreditRequest, CalculateCarbonCreditResponse, CalculateEmissionRequest, CalculateEmissionResponse, CarbonCreditBreakdown};
use crate::models::waste::{QualityFactor, VerificationMode};
use super::ProcessingMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub credits_kg: f64,
    pub credits_tonnes: f64,
    pub breakdown: CarbonCreditBreakdown,
}

pub struct CarbonCalculator;

impl CarbonCalculator {
    pub fn calculate_emission(req: CalculateEmissionRequest) -> Result<CalculateEmissionResponse, AppError> {
        let category = req.waste_category.to_lowercase();
        let subtype = req.waste_subtype.to_lowercase();

        let factor = get_emission_factor(&category, &subtype)
            .ok_or_else(|| AppError::InvalidCategory(format!("Unknown waste type: {} / {}", req.waste_category, req.waste_subtype)))?;

        let waste_emission = req.weight_kg * factor.ef;

        let transport_emission = req.distance_km
            .map(|d| d * 0.21)
            .unwrap_or(0.0);

        let processing_method_str = match req.processing_method {
            ProcessingMethod::Landfill => "landfill",
            ProcessingMethod::Composting => "composting",
            ProcessingMethod::Biogas => "biogas",
            ProcessingMethod::Recycling => "recycling",
            ProcessingMethod::EnergyRecovery => "energy_recovery",
            ProcessingMethod::Biochar => "biochar",
        };
        let processing_emission = req.weight_kg * get_processing_factor(&category, processing_method_str).abs() * 0.1;

        let energy_emission = req.energy_kwh
            .map(|e| e * get_grid_emission_factor(&req.state_code))
            .unwrap_or(0.0);

        let total_emission = waste_emission + transport_emission + processing_emission + energy_emission;

        Ok(CalculateEmissionResponse {
            total_emission_kg: total_emission,
            waste_emission_kg: waste_emission,
            transport_emission_kg: transport_emission,
            processing_emission_kg: processing_emission,
            energy_emission_kg: energy_emission,
        })
    }

    pub fn calculate_credit(req: CalculateCarbonCreditRequest) -> Result<CalculateCarbonCreditResponse, AppError> {
        let category = req.waste_category.to_lowercase();
        let subtype = req.waste_subtype.to_lowercase();

        let factor = get_emission_factor(&category, &subtype)
            .ok_or_else(|| AppError::InvalidCategory(format!("Unknown waste type: {} / {}", req.waste_category, req.waste_subtype)))?;

        let verification_score = req.verification_mode.score();
        let locality_factor = get_locality_factor(&req.state_code);
        let quality_factor = req.quality_factor
            .map(|q| q.value())
            .unwrap_or(1.0);

        let baseline_scenario = req.baseline_scenario.unwrap_or(BaselineScenario::Landfill);
        let baseline_emission = Self::calculate_baseline_emission(
            &category,
            &subtype,
            req.weight_kg,
            &baseline_scenario,
        );

        let processing_method_str = match req.processing_method {
            ProcessingMethod::Landfill => "landfill",
            ProcessingMethod::Composting => "composting",
            ProcessingMethod::Biogas => "biogas",
            ProcessingMethod::Recycling => "recycling",
            ProcessingMethod::EnergyRecovery => "energy_recovery",
            ProcessingMethod::Biochar => "biochar",
        };
        let processing_factor = get_processing_factor(&category, processing_method_str);

        let actual_emission = req.weight_kg * factor.ef * processing_factor.abs();

        let net_reduction = (baseline_emission - actual_emission).max(0.0);

        let additionality_factor = Self::calculate_additionality_factor(&req.processing_method, &req.verification_mode);
        let leakage_factor = 0.95;

        let base_credits = net_reduction * factor.multiplier;
        let final_credits = base_credits * verification_score * locality_factor * quality_factor * additionality_factor * leakage_factor;

        let credits_kg = final_credits.max(0.0);
        let credits_tonnes = credits_kg / 1000.0;
        let token_units = (credits_kg * 1000.0).round() as i64;

        Ok(CalculateCarbonCreditResponse {
            credits_kg,
            credits_tonnes,
            baseline_emission_kg: baseline_emission,
            actual_emission_kg: actual_emission,
            net_reduction_kg: net_reduction,
            token_units,
            breakdown: CarbonCreditBreakdown {
                waste_weight_kg: req.weight_kg,
                emission_factor: factor.ef,
                credit_multiplier: factor.multiplier,
                verification_score,
                locality_factor,
                quality_factor,
                additionality_factor,
                leakage_factor,
            },
        })
    }

    fn calculate_baseline_emission(category: &str, subtype: &str, weight_kg: f64, scenario: &BaselineScenario) -> f64 {
        let base_factor = get_emission_factor(category, subtype)
            .map(|f| f.ef)
            .unwrap_or(1.0);

        match scenario {
            BaselineScenario::Landfill => weight_kg * base_factor * 1.0,
            BaselineScenario::Incineration => weight_kg * base_factor * 0.8,
            BaselineScenario::OpenBurning => weight_kg * base_factor * 1.5,
        }
    }

    fn calculate_additionality_factor(processing_method: &ProcessingMethod, verification_mode: &VerificationMode) -> f64 {
        let method_factor = match processing_method {
            ProcessingMethod::Landfill => 0.5,
            ProcessingMethod::Composting => 0.8,
            ProcessingMethod::Biogas => 1.0,
            ProcessingMethod::Recycling => 1.0,
            ProcessingMethod::EnergyRecovery => 0.9,
            ProcessingMethod::Biochar => 1.2,
        };

        let verification_factor = match verification_mode {
            VerificationMode::SelfReported => 0.7,
            VerificationMode::AI => 0.9,
            VerificationMode::Facility => 1.0,
        };

        method_factor * verification_factor
    }
}

pub fn calculate_credits(
    category: &str,
    subtype: &str,
    weight_kg: f64,
    verification_mode: VerificationMode,
    quality_factor: Option<QualityFactor>,
    state_code: &str,
    baseline_scenario: Option<BaselineScenario>,
    processing_method: ProcessingMethod,
) -> Result<CalculationResult, AppError> {
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

    Ok(CalculationResult {
        credits_kg: response.credits_kg,
        credits_tonnes: response.credits_tonnes,
        breakdown: response.breakdown,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_bottle_credit_calculation() {
        let req = CalculateCarbonCreditRequest {
            waste_category: "recyclables".to_string(),
            waste_subtype: "pet_bottles".to_string(),
            weight_kg: 3.0,
            processing_method: ProcessingMethod::Recycling,
            verification_mode: VerificationMode::AI,
            quality_factor: Some(QualityFactor::Clean),
            state_code: "IN-MH".to_string(),
            baseline_scenario: Some(BaselineScenario::Landfill),
        };

        let result = CarbonCalculator::calculate_credit(req).unwrap();
        assert!(result.credits_kg > 0.0);
        assert!(result.token_units > 0);
    }

    #[test]
    fn test_aluminum_cans_high_credit() {
        let req = CalculateCarbonCreditRequest {
            waste_category: "recyclables".to_string(),
            waste_subtype: "aluminum_cans".to_string(),
            weight_kg: 1.0,
            processing_method: ProcessingMethod::Recycling,
            verification_mode: VerificationMode::Facility,
            quality_factor: Some(QualityFactor::Clean),
            state_code: "IN-DL".to_string(),
            baseline_scenario: Some(BaselineScenario::Landfill),
        };

        let result = CarbonCalculator::calculate_credit(req).unwrap();
        assert!(result.credits_kg > 40.0);
    }

    #[test]
    fn test_unknown_waste_type() {
        let req = CalculateCarbonCreditRequest {
            waste_category: "unknown".to_string(),
            waste_subtype: "unknown".to_string(),
            weight_kg: 1.0,
            processing_method: ProcessingMethod::Recycling,
            verification_mode: VerificationMode::SelfReported,
            quality_factor: None,
            state_code: "IN-MH".to_string(),
            baseline_scenario: None,
        };

        let result = CarbonCalculator::calculate_credit(req);
        assert!(result.is_err());
    }

    #[test]
    fn test_emission_calculation() {
        let req = CalculateEmissionRequest {
            waste_category: "organic".to_string(),
            waste_subtype: "vegetable_scraps".to_string(),
            weight_kg: 10.0,
            processing_method: ProcessingMethod::Composting,
            distance_km: Some(5.0),
            energy_kwh: Some(2.0),
            state_code: "IN-MH".to_string(),
        };

        let result = CarbonCalculator::calculate_emission(req).unwrap();
        assert!(result.waste_emission_kg > 0.0);
    }
}