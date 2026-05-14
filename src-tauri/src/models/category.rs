use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub household_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: String,
    pub parent_id: Option<String>,
    pub color: i64,
    pub icon_key: String,
    pub sort_order: i32,
    pub updated_at: i64,
    pub is_deleted: bool,
}

pub const TYPE_INCOME: &str = "INCOME";
pub const TYPE_EXPENSE: &str = "EXPENSE";

pub fn is_valid_type(t: &str) -> bool {
    matches!(t, TYPE_INCOME | TYPE_EXPENSE)
}
