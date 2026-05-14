use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub household_id: String,
    pub occurred_at: i64,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub amount_minor: i64,
    pub currency: String,
    pub account_id: String,
    pub to_account_id: Option<String>,
    pub category_id: Option<String>,
    pub merchant_id: Option<String>,
    pub note: Option<String>,
    pub created_by: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub source_type: String,
    pub receipt_id: Option<String>,
    pub is_deleted: bool,
}

pub const TYPE_INCOME: &str = "INCOME";
pub const TYPE_EXPENSE: &str = "EXPENSE";
pub const TYPE_TRANSFER: &str = "TRANSFER";

pub const SRC_MANUAL: &str = "MANUAL";
pub const SRC_SMS: &str = "SMS";
pub const SRC_QR: &str = "QR";
pub const SRC_IMPORT: &str = "IMPORT";

pub fn is_valid_type(t: &str) -> bool {
    matches!(t, TYPE_INCOME | TYPE_EXPENSE | TYPE_TRANSFER)
}
