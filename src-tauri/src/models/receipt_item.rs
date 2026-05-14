use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptItem {
    pub id: String,
    pub transaction_id: String,
    pub name: String,
    pub price_minor: i64,
    pub qty: f64,
    pub unit: Option<String>,
    pub fns_raw: Option<String>,
}
