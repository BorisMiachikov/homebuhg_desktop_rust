use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::account::{is_valid_type, Account};
use crate::repository::{accounts, now_ms};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInput {
    pub id: Option<String>,
    pub household_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub currency: Option<String>,
    pub balance_minor: Option<i64>,
    pub credit_limit_minor: Option<i64>,
    pub grace_period_days: Option<i32>,
    pub payment_due_day: Option<i32>,
    pub color: Option<i64>,
    pub icon_key: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalsResponse {
    pub total_minor: i64,
}

#[tauri::command]
pub fn accounts_list(db: State<'_, DbState>, household_id: String) -> AppResult<Vec<Account>> {
    let conn = db.lock();
    accounts::list_by_household(&conn, &household_id)
}

#[tauri::command]
pub fn accounts_get(db: State<'_, DbState>, id: String) -> AppResult<Option<Account>> {
    let conn = db.lock();
    accounts::get(&conn, &id)
}

#[tauri::command]
pub fn accounts_upsert(db: State<'_, DbState>, input: AccountInput) -> AppResult<Account> {
    if !is_valid_type(&input.account_type) {
        return Err(AppError::InvalidArg(format!(
            "invalid account type: {}",
            input.account_type
        )));
    }
    let now = now_ms();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let a = Account {
        id: id.clone(),
        household_id: input.household_id,
        name: input.name.trim().to_string(),
        account_type: input.account_type,
        currency: input.currency.unwrap_or_else(|| "RUB".into()),
        balance_minor: input.balance_minor.unwrap_or(0),
        credit_limit_minor: input.credit_limit_minor,
        grace_period_days: input.grace_period_days,
        payment_due_day: input.payment_due_day,
        color: input.color.unwrap_or(0xFF607D8B),
        icon_key: input.icon_key.unwrap_or_else(|| "credit_card".into()),
        is_archived: false,
        updated_at: now,
    };
    let conn = db.lock();
    accounts::upsert(&conn, &a)?;
    accounts::get(&conn, &id)?.ok_or_else(|| AppError::NotFound(id))
}

#[tauri::command]
pub fn accounts_archive(db: State<'_, DbState>, id: String, archived: bool) -> AppResult<()> {
    let conn = db.lock();
    accounts::set_archived(&conn, &id, archived, now_ms())
}

#[tauri::command]
pub fn accounts_total(db: State<'_, DbState>, household_id: String) -> AppResult<TotalsResponse> {
    let conn = db.lock();
    let total = accounts::total_balance(&conn, &household_id)?;
    Ok(TotalsResponse { total_minor: total })
}
