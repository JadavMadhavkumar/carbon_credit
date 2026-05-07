use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Invoice {
    pub id: Uuid,
    pub shopkeeper_id: Uuid,
    pub invoice_date: DateTime<Utc>,
    pub total_amount: Decimal,
    pub plastic_product_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InvoiceItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewInvoice {
    pub shopkeeper_id: Uuid,
    pub items: Vec<InvoiceItem>,
}
