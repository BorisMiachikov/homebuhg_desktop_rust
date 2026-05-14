use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringRule {
    pub id: String,
    pub household_id: String,
    pub template_json: String,
    pub rrule: String,
    pub next_run_at: i64,
    pub last_run_at: Option<i64>,
    pub is_active: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringTemplate {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub amount_minor: i64,
    pub currency: String,
    pub account_id: String,
    pub to_account_id: Option<String>,
    pub category_id: Option<String>,
    pub note: Option<String>,
}
