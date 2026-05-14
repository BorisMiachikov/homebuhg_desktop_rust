use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    pub id: String,
    pub household_id: String,
    pub name: String,
    pub default_category_id: Option<String>,
    pub last_used_at: i64,
    pub updated_at: i64,
    pub is_deleted: bool,
}
