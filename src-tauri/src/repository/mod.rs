pub mod accounts;
pub mod budgets;
pub mod categories;
pub mod households;
pub mod merchants;
pub mod receipt_items;
pub mod recurring;
pub mod transactions;

pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
