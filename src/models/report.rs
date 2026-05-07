use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub household_id: Option<Uuid>,
    pub municipality_id: Option<Uuid>,
    pub report_type: ReportType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_credits_kg: f64,
    pub total_submissions: i64,
    pub category_breakdown: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportType {
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "municipality")]
    Municipality,
    #[serde(rename = "national")]
    National,
    #[serde(rename = "analytics")]
    Analytics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    pub total_credits_kg: f64,
    pub total_credits_tonnes: f64,
    pub total_submissions: i64,
    pub average_credits_per_submission_kg: f64,
    pub category_breakdown: Vec<CategoryAnalytics>,
    pub state_breakdown: Vec<StateAnalytics>,
    pub trend_data: Vec<TrendPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryAnalytics {
    pub category: String,
    pub total_weight_kg: f64,
    pub total_credits_kg: f64,
    pub submission_count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateAnalytics {
    pub state_code: String,
    pub total_credits_kg: f64,
    pub submission_count: i64,
    pub participation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub period: String,
    pub credits_kg: f64,
    pub submissions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRequest {
    pub aggregation_level: AggregationLevel,
    pub state_code: Option<String>,
    pub municipality_id: Option<Uuid>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AggregationLevel {
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "municipality")]
    Municipality,
    #[serde(rename = "state")]
    State,
    #[serde(rename = "national")]
    National,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResponse {
    pub aggregation_level: AggregationLevel,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_credits_kg: f64,
    pub total_submissions: i64,
    pub active_households: i64,
    pub participation_rate: f64,
    pub per_capita_index: f64,
    pub participation_bonus: f64,
    pub final_credits_kg: f64,
    pub breakdown: Vec<AggregationBreakdownItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationBreakdownItem {
    pub category: String,
    pub credits_kg: f64,
    pub percentage: f64,
}