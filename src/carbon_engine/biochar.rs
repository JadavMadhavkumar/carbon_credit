use crate::carbon_engine::factors::get_grid_emission_factor;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use crate::models::carbon::{BiocharBreakdown, BiocharProcessRequest, BiocharProcessResponse};

pub struct BiocharProcessor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiocharParameters {
    pub biomass_type: String,
    pub biomass_weight_kg: f64,
    pub pyrolysis_temp_c: f64,
    pub residence_time_min: i32,
}

impl BiocharProcessor {
    pub fn process(req: BiocharProcessRequest) -> Result<BiocharProcessResponse, AppError> {
        let biomass_type = req.biomass_type.to_lowercase();
        let biomass_weight = req.biomass_weight_kg;

        if biomass_weight <= 0.0 {
            return Err(AppError::Validation("Biomass weight must be positive".to_string()));
        }

        if req.pyrolysis_temperature_c < 200.0 || req.pyrolysis_temperature_c > 900.0 {
            return Err(AppError::Validation("Pyrolysis temperature must be between 200-900°C".to_string()));
        }

        let (yield_rate, carbon_content, stability_factor) = Self::get_biochar_parameters(&biomass_type, req.pyrolysis_temperature_c);

        let biochar_output = biomass_weight * yield_rate;

        let carbon_retained = biochar_output * carbon_content * stability_factor;

        let grid_ef = get_grid_emission_factor(&req.state_code);

        let energy_displacement_credits = Self::calculate_energy_credits(biochar_output, req.pyrolysis_temperature_c, grid_ef);

        let total_carbon_credits = carbon_retained + energy_displacement_credits;

        let credits_kg = total_carbon_credits.max(0.0);
        let credits_tonnes = credits_kg / 1000.0;
        let token_units = (credits_kg * 1000.0).round() as i64;

        Ok(BiocharProcessResponse {
            biochar_output_kg: biochar_output,
            carbon_sequestered_kg: carbon_retained,
            carbon_credits_kg: credits_kg,
            carbon_credits_tonnes: credits_tonnes,
            token_units,
            breakdown: BiocharBreakdown {
                biomass_input_kg: biomass_weight,
                biochar_yield_rate: yield_rate,
                carbon_content,
                stability_factor,
                carbon_retained_kg: carbon_retained,
                emission_avoided_kg: energy_displacement_credits,
            },
        })
    }

    fn get_biochar_parameters(biomass_type: &str, temperature_c: f64) -> (f64, f64, f64) {
        let temp_factor = if temperature_c < 400.0 {
            0.7
        } else if temperature_c < 550.0 {
            0.85
        } else if temperature_c < 700.0 {
            1.0
        } else {
            0.95
        };

        match biomass_type {
            "wood" | "woody" => (
                0.25 * temp_factor,
                0.80,
                0.85,
            ),
            "agricultural_waste" | "agri_waste" | "straw" => (
                0.30 * temp_factor,
                0.70,
                0.75,
            ),
            "coconut_shell" | "coconut" | "shell" => (
                0.28 * temp_factor,
                0.75,
                0.90,
            ),
            "bamboo" => (
                0.27 * temp_factor,
                0.72,
                0.80,
            ),
            "grass" | "leaves" | "green_waste" => (
                0.35 * temp_factor,
                0.65,
                0.70,
            ),
            "sawdust" | "saw_dust" | "wood_chips" => (
                0.22 * temp_factor,
                0.78,
                0.82,
            ),
            "food_waste" | "organic_waste" => (
                0.20 * temp_factor,
                0.60,
                0.65,
            ),
            _ => (
                0.25 * temp_factor,
                0.70,
                0.75,
            ),
        }
    }

    fn calculate_energy_credits(biochar_kg: f64, temperature_c: f64, grid_ef: f64) -> f64 {
        let energy_recovery_factor = if temperature_c >= 500.0 { 0.4 } else { 0.2 };
        let energy_mj_per_kg = 25.0;
        let kwh_per_mj = 1.0 / 3.6;

        let energy_kwh = biochar_kg * energy_recovery_factor * energy_mj_per_kg * kwh_per_mj;
        energy_kwh * grid_ef * 0.9
    }

    pub fn calculate_sequestration(
        biomass_input_kg: f64,
        biomass_type: &str,
        temperature_c: f64,
    ) -> Result<SequestrationResult, AppError> {
        let (yield_rate, carbon_content, stability_factor) = Self::get_biochar_parameters(biomass_type, temperature_c);

        let biochar_output = biomass_input_kg * yield_rate;
        let carbon_sequestered = biochar_output * carbon_content * stability_factor;

        let co2_equivalent = carbon_sequestered * 44.0 / 12.0;

        Ok(SequestrationResult {
            biochar_output_kg: biochar_output,
            carbon_sequestered_kg: carbon_sequestered,
            co2_equivalent_kg: co2_equivalent,
            carbon_credits_kg: co2_equivalent,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequestrationResult {
    pub biochar_output_kg: f64,
    pub carbon_sequestered_kg: f64,
    pub co2_equivalent_kg: f64,
    pub carbon_credits_kg: f64,
}

pub fn calculate_carbon_removed(
    biomass_kg: f64,
    conversion_rate: f64,
    carbon_content: f64,
    stability_factor: f64,
) -> f64 {
    let biochar = biomass_kg * conversion_rate;
    let carbon = biochar * carbon_content;
    carbon * stability_factor
}

pub fn calculate_carbon_credits_from_sequestration(carbon_kg: f64) -> (f64, i64) {
    let co2_equivalent = carbon_kg * 44.0 / 12.0;
    let token_units = (co2_equivalent * 1000.0).round() as i64;
    (co2_equivalent, token_units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biochar_wood_processing() {
        let req = BiocharProcessRequest {
            biomass_type: "wood".to_string(),
            biomass_weight_kg: 100.0,
            pyrolysis_temperature_c: 550.0,
            residence_time_minutes: 30,
            state_code: "IN-MH".to_string(),
        };

        let result = BiocharProcessor::process(req).unwrap();
        assert!(result.biochar_output_kg > 0.0);
        assert!(result.carbon_credits_kg > 0.0);
    }

    #[test]
    fn test_biochar_invalid_temperature() {
        let req = BiocharProcessRequest {
            biomass_type: "wood".to_string(),
            biomass_weight_kg: 100.0,
            pyrolysis_temperature_c: 100.0,
            residence_time_minutes: 30,
            state_code: "IN-MH".to_string(),
        };

        let result = BiocharProcessor::process(req);
        assert!(result.is_err());
    }

    #[test]
    fn test_sequestration_calculation() {
        let result = BiocharProcessor::calculate_sequestration(100.0, "wood", 550.0).unwrap();
        assert!(result.biochar_output_kg > 0.0);
        assert!(result.co2_equivalent_kg > 0.0);
    }

    #[test]
    fn test_carbon_removed_formula() {
        let removed = calculate_carbon_removed(100.0, 0.25, 0.80, 0.85);
        assert!(removed > 0.0);
    }
}
