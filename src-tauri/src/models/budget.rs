use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: String,
    pub household_id: String,
    pub category_id: String,
    pub period: String,
    pub limit_minor: i64,
    pub currency: String,
    pub start_date: i64,
    pub is_rolling: bool,
    pub updated_at: i64,
    pub is_deleted: bool,
}

pub const PERIOD_WEEK: &str = "WEEK";
pub const PERIOD_MONTH: &str = "MONTH";
pub const PERIOD_YEAR: &str = "YEAR";

pub fn is_valid_period(p: &str) -> bool {
    matches!(p, PERIOD_WEEK | PERIOD_MONTH | PERIOD_YEAR)
}
