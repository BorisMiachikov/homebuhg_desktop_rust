use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub household_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub currency: String,
    pub balance_minor: i64,
    pub credit_limit_minor: Option<i64>,
    pub grace_period_days: Option<i32>,
    pub payment_due_day: Option<i32>,
    pub color: i64,
    pub icon_key: String,
    pub is_archived: bool,
    pub updated_at: i64,
}

pub const TYPE_CARD_DEBIT: &str = "CARD_DEBIT";
pub const TYPE_CARD_CREDIT: &str = "CARD_CREDIT";
pub const TYPE_CASH: &str = "CASH";

pub fn is_valid_type(t: &str) -> bool {
    matches!(t, TYPE_CARD_DEBIT | TYPE_CARD_CREDIT | TYPE_CASH)
}
