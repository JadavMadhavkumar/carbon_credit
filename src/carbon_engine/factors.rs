use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmissionFactor {
    pub ef: f64,
    pub multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalityFactor {
    pub state_code: String,
    pub factor: f64,
    pub grid_emission: f64,
}

struct FactorMap {
    map: DashMap<(String, String), EmissionFactor>,
}

impl FactorMap {
    fn new() -> Self {
        let map = DashMap::new();
        map.insert(("organic".to_string(), "vegetable_scraps".to_string()), EmissionFactor { ef: 0.57, multiplier: 1.0 });
        map.insert(("organic".to_string(), "fruit_peels".to_string()), EmissionFactor { ef: 0.53, multiplier: 1.0 });
        map.insert(("organic".to_string(), "meat_dairy".to_string()), EmissionFactor { ef: 2.85, multiplier: 1.8 });
        map.insert(("organic".to_string(), "cooked_food".to_string()), EmissionFactor { ef: 1.12, multiplier: 1.3 });
        map.insert(("organic".to_string(), "garden_waste".to_string()), EmissionFactor { ef: 0.46, multiplier: 0.9 });
        map.insert(("organic".to_string(), "rice_grains".to_string()), EmissionFactor { ef: 0.89, multiplier: 1.1 });
        map.insert(("organic".to_string(), "used_cooking_oil".to_string()), EmissionFactor { ef: 2.47, multiplier: 3.5 });
        map.insert(("organic".to_string(), "bread_cereals".to_string()), EmissionFactor { ef: 0.72, multiplier: 1.0 });
        map.insert(("organic".to_string(), "tea_coffee".to_string()), EmissionFactor { ef: 0.38, multiplier: 0.8 });

        map.insert(("recyclables".to_string(), "newspaper".to_string()), EmissionFactor { ef: 1.02, multiplier: 1.2 });
        map.insert(("recyclables".to_string(), "cardboard".to_string()), EmissionFactor { ef: 0.89, multiplier: 1.1 });
        map.insert(("recyclables".to_string(), "contaminated_paper".to_string()), EmissionFactor { ef: 0.40, multiplier: 0.5 });
        map.insert(("recyclables".to_string(), "pet_bottles".to_string()), EmissionFactor { ef: 1.78, multiplier: 2.1 });
        map.insert(("recyclables".to_string(), "hdpe_plastic".to_string()), EmissionFactor { ef: 1.62, multiplier: 1.9 });
        map.insert(("recyclables".to_string(), "mixed_plastic".to_string()), EmissionFactor { ef: 1.10, multiplier: 0.8 });
        map.insert(("recyclables".to_string(), "single_use_plastic".to_string()), EmissionFactor { ef: 0.95, multiplier: 0.7 });
        map.insert(("recyclables".to_string(), "clear_glass".to_string()), EmissionFactor { ef: 0.31, multiplier: 0.8 });
        map.insert(("recyclables".to_string(), "colored_glass".to_string()), EmissionFactor { ef: 0.31, multiplier: 0.7 });
        map.insert(("recyclables".to_string(), "aluminum_cans".to_string()), EmissionFactor { ef: 8.14, multiplier: 6.0 });
        map.insert(("recyclables".to_string(), "steel_cans".to_string()), EmissionFactor { ef: 1.46, multiplier: 1.8 });
        map.insert(("recyclables".to_string(), "copper_wire".to_string()), EmissionFactor { ef: 2.60, multiplier: 2.5 });

        map.insert(("ewaste".to_string(), "smartphone".to_string()), EmissionFactor { ef: 45.0, multiplier: 8.0 });
        map.insert(("ewaste".to_string(), "laptop".to_string()), EmissionFactor { ef: 320.0, multiplier: 10.0 });
        map.insert(("ewaste".to_string(), "desktop_pc".to_string()), EmissionFactor { ef: 260.0, multiplier: 9.0 });
        map.insert(("ewaste".to_string(), "tablet".to_string()), EmissionFactor { ef: 120.0, multiplier: 8.5 });
        map.insert(("ewaste".to_string(), "crt_monitor".to_string()), EmissionFactor { ef: 70.0, multiplier: 5.0 });
        map.insert(("ewaste".to_string(), "lcd_monitor".to_string()), EmissionFactor { ef: 70.0, multiplier: 5.5 });
        map.insert(("ewaste".to_string(), "liion_battery".to_string()), EmissionFactor { ef: 12.6, multiplier: 7.0 });
        map.insert(("ewaste".to_string(), "cfl_bulb".to_string()), EmissionFactor { ef: 5.3, multiplier: 4.0 });
        map.insert(("ewaste".to_string(), "refrigerator".to_string()), EmissionFactor { ef: 130.0, multiplier: 6.0 });
        map.insert(("ewaste".to_string(), "air_conditioner".to_string()), EmissionFactor { ef: 180.0, multiplier: 7.0 });

        map.insert(("textiles".to_string(), "cotton".to_string()), EmissionFactor { ef: 6.80, multiplier: 3.0 });
        map.insert(("textiles".to_string(), "synthetic".to_string()), EmissionFactor { ef: 9.50, multiplier: 2.5 });
        map.insert(("textiles".to_string(), "wool".to_string()), EmissionFactor { ef: 90.0, multiplier: 4.0 });
        map.insert(("textiles".to_string(), "polyester".to_string()), EmissionFactor { ef: 9.52, multiplier: 2.5 });
        map.insert(("textiles".to_string(), "nylon".to_string()), EmissionFactor { ef: 9.80, multiplier: 2.5 });
        map.insert(("textiles".to_string(), "denim".to_string()), EmissionFactor { ef: 8.10, multiplier: 3.5 });
        map.insert(("textiles".to_string(), "leather".to_string()), EmissionFactor { ef: 17.0, multiplier: 2.0 });
        map.insert(("textiles".to_string(), "blended".to_string()), EmissionFactor { ef: 7.50, multiplier: 2.0 });

        map.insert(("hazardous".to_string(), "paints_solvents".to_string()), EmissionFactor { ef: 2.2, multiplier: 1.5 });
        map.insert(("hazardous".to_string(), "medicines".to_string()), EmissionFactor { ef: 3.1, multiplier: 2.0 });
        map.insert(("hazardous".to_string(), "pesticides".to_string()), EmissionFactor { ef: 4.8, multiplier: 2.5 });
        map.insert(("hazardous".to_string(), "motor_oil".to_string()), EmissionFactor { ef: 3.3, multiplier: 1.8 });
        map.insert(("hazardous".to_string(), "lead_battery".to_string()), EmissionFactor { ef: 5.2, multiplier: 2.0 });
        map.insert(("hazardous".to_string(), "fluorescent_tubes".to_string()), EmissionFactor { ef: 5.3, multiplier: 4.0 });

        map.insert(("bulky".to_string(), "wardrobe_wood".to_string()), EmissionFactor { ef: 2.1, multiplier: 2.5 });
        map.insert(("bulky".to_string(), "almirah_steel".to_string()), EmissionFactor { ef: 1.8, multiplier: 2.0 });
        map.insert(("bulky".to_string(), "sofa".to_string()), EmissionFactor { ef: 3.2, multiplier: 2.0 });
        map.insert(("bulky".to_string(), "dining_table".to_string()), EmissionFactor { ef: 2.4, multiplier: 2.5 });
        map.insert(("bulky".to_string(), "mattress".to_string()), EmissionFactor { ef: 2.8, multiplier: 1.5 });

        Self { map }
    }

    fn get(&self, category: &str, subtype: &str) -> Option<EmissionFactor> {
        let key = (category.to_lowercase(), subtype.to_lowercase());
        self.map.get(&key).map(|r| *r)
    }
}

static EMISSION_FACTORS: Lazy<FactorMap> = Lazy::new(FactorMap::new);

struct LocalityMap {
    map: DashMap<String, LocalityFactor>,
}

impl LocalityMap {
    fn new() -> Self {
        let map = DashMap::new();
        map.insert("IN-MH".to_string(), LocalityFactor { state_code: "IN-MH".to_string(), factor: 0.98, grid_emission: 0.82 });
        map.insert("IN-DL".to_string(), LocalityFactor { state_code: "IN-DL".to_string(), factor: 1.02, grid_emission: 0.87 });
        map.insert("IN-KA".to_string(), LocalityFactor { state_code: "IN-KA".to_string(), factor: 0.94, grid_emission: 0.74 });
        map.insert("IN-TN".to_string(), LocalityFactor { state_code: "IN-TN".to_string(), factor: 0.97, grid_emission: 0.78 });
        map.insert("IN-GJ".to_string(), LocalityFactor { state_code: "IN-GJ".to_string(), factor: 1.01, grid_emission: 0.88 });
        map.insert("IN-RJ".to_string(), LocalityFactor { state_code: "IN-RJ".to_string(), factor: 1.05, grid_emission: 0.92 });
        map.insert("IN-UP".to_string(), LocalityFactor { state_code: "IN-UP".to_string(), factor: 1.08, grid_emission: 0.96 });
        map.insert("IN-WB".to_string(), LocalityFactor { state_code: "IN-WB".to_string(), factor: 1.03, grid_emission: 0.99 });
        map.insert("IN-TS".to_string(), LocalityFactor { state_code: "IN-TS".to_string(), factor: 0.99, grid_emission: 0.81 });
        map.insert("IN-KL".to_string(), LocalityFactor { state_code: "IN-KL".to_string(), factor: 0.91, grid_emission: 0.75 });
        map.insert("IN-PB".to_string(), LocalityFactor { state_code: "IN-PB".to_string(), factor: 0.96, grid_emission: 0.85 });
        map.insert("IN-MP".to_string(), LocalityFactor { state_code: "IN-MP".to_string(), factor: 1.06, grid_emission: 0.90 });
        Self { map }
    }

    fn get(&self, state_code: &str) -> Option<LocalityFactor> {
        self.map.get(state_code).map(|r| r.value().clone())
    }
}

static LOCALITY_FACTORS: Lazy<LocalityMap> = Lazy::new(LocalityMap::new);

struct ProcessingMap {
    map: DashMap<(String, String), f64>,
}

impl ProcessingMap {
    fn new() -> Self {
        let map = DashMap::new();
        map.insert(("organic".to_string(), "composting".to_string()), 0.57);
        map.insert(("organic".to_string(), "biogas".to_string()), 1.12);
        map.insert(("organic".to_string(), "landfill".to_string()), -0.252);
        map.insert(("organic".to_string(), "biochar".to_string()), 1.5);
        map.insert(("recyclables".to_string(), "recycling".to_string()), 1.0);
        map.insert(("recyclables".to_string(), "landfill".to_string()), 0.0);
        map.insert(("recyclables".to_string(), "energy_recovery".to_string()), 0.8);
        map.insert(("ewaste".to_string(), "certified_recycling".to_string()), 1.0);
        map.insert(("textiles".to_string(), "reuse".to_string()), 1.0);
        map.insert(("textiles".to_string(), "recycling".to_string()), 0.8);
        map.insert(("textiles".to_string(), "energy_recovery".to_string()), 0.6);
        Self { map }
    }

    fn get(&self, category: &str, method: &str) -> f64 {
        let key = (category.to_lowercase(), method.to_lowercase());
        self.map.get(&key).map(|v| *v).unwrap_or(1.0)
    }
}

static PROCESSING_METHODS: Lazy<ProcessingMap> = Lazy::new(ProcessingMap::new);

pub fn get_emission_factor(category: &str, subtype: &str) -> Option<EmissionFactor> {
    EMISSION_FACTORS.get(category, subtype)
}

pub fn get_locality_factor(state_code: &str) -> f64 {
    LOCALITY_FACTORS.get(state_code).map(|lf| lf.factor).unwrap_or(1.0)
}

pub fn get_grid_emission_factor(state_code: &str) -> f64 {
    LOCALITY_FACTORS.get(state_code).map(|lf| lf.grid_emission).unwrap_or(0.82)
}

pub fn get_processing_factor(category: &str, method: &str) -> f64 {
    PROCESSING_METHODS.get(category, method)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_bottle_factor() {
        let factor = get_emission_factor("recyclables", "pet_bottles");
        assert!(factor.is_some());
        let f = factor.unwrap();
        assert!((f.ef - 1.78).abs() < 0.001);
        assert!((f.multiplier - 2.1).abs() < 0.001);
    }

    #[test]
    fn test_locality_factor() {
        let lf = get_locality_factor("IN-MH");
        assert!((lf - 0.98).abs() < 0.001);
    }

    #[test]
    fn test_unknown_factor_returns_none() {
        let factor = get_emission_factor("unknown", "unknown");
        assert!(factor.is_none());
    }
}