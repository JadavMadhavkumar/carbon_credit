use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationLog {
    pub id: String,
    pub user_id: String,
    pub waste_category: String,
    pub waste_subtype: String,
    pub weight_kg: f64,
    pub processing_method: String,
    pub credits_kg: f64,
    pub verification_mode: String,
    pub state_code: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditIssuanceLog {
    pub id: String,
    pub user_id: String,
    pub credit_amount_kg: f64,
    pub token_id: Option<String>,
    pub submission_id: Option<String>,
    pub status: String,
    pub issued_at: DateTime<Utc>,
}

pub struct AuditLogger {
    logs: Vec<AuditLog>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log_action(
        &mut self,
        user_id: Option<&str>,
        action: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        details: Option<serde_json::Value>,
    ) {
        let log = AuditLog {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.map(String::from),
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(String::from),
            details,
            ip_address: None,
            timestamp: Utc::now(),
        };
        self.logs.push(log);
    }

    pub fn log_calculation(
        &mut self,
        user_id: &str,
        category: &str,
        subtype: &str,
        weight_kg: f64,
        method: &str,
        credits_kg: f64,
        verification: &str,
        state_code: &str,
    ) {
        let details = serde_json::json!({
            "waste_category": category,
            "waste_subtype": subtype,
            "weight_kg": weight_kg,
            "processing_method": method,
            "credits_kg": credits_kg,
            "verification_mode": verification,
            "state_code": state_code,
        });
        
        self.log_action(
            Some(user_id),
            "calculate_credits",
            "calculation",
            None,
            Some(details),
        );
    }

    pub fn log_credit_issuance(
        &mut self,
        user_id: &str,
        amount_kg: f64,
        token_id: Option<&str>,
        submission_id: Option<&str>,
    ) {
        let details = serde_json::json!({
            "credit_amount_kg": amount_kg,
            "token_id": token_id,
            "submission_id": submission_id,
        });
        
        self.log_action(
            Some(user_id),
            "issue_credit",
            "credit",
            token_id,
            Some(details),
        );
    }

    pub fn log_user_action(&mut self, user_id: &str, action: &str) {
        self.log_action(Some(user_id), action, "user", None, None);
    }

    pub fn get_logs(&self) -> &[AuditLog] {
        &self.logs
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}