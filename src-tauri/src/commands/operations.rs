use crate::db::DbState;
use crate::domain::balance;
use crate::error::{AppError, AppResult};
use crate::models::receipt_item::ReceiptItem;
use crate::models::transaction::{is_valid_type, Transaction, SRC_MANUAL};
use crate::repository::{accounts, now_ms, receipt_items, transactions};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptItemInput {
    pub id: Option<String>,
    pub name: String,
    pub price_minor: i64,
    pub qty: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
    pub id: Option<String>,
    pub household_id: String,
    pub occurred_at: i64,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub amount_minor: i64,
    pub currency: Option<String>,
    pub account_id: String,
    pub to_account_id: Option<String>,
    pub category_id: Option<String>,
    pub merchant_id: Option<String>,
    pub note: Option<String>,
    pub created_by: Option<String>,
    pub items: Option<Vec<ReceiptItemInput>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDetail {
    pub transaction: Transaction,
    pub items: Vec<ReceiptItem>,
}

#[tauri::command]
pub fn operations_list(
    db: State<'_, DbState>,
    household_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
) -> AppResult<Vec<Transaction>> {
    let conn = db.lock();
    transactions::list_by_household(
        &conn,
        &household_id,
        limit.unwrap_or(500),
        offset.unwrap_or(0),
    )
}

#[tauri::command]
pub fn operations_get(db: State<'_, DbState>, id: String) -> AppResult<Option<OperationDetail>> {
    let conn = db.lock();
    let tx = match transactions::get(&conn, &id)? {
        Some(t) => t,
        None => return Ok(None),
    };
    let items = receipt_items::list_by_tx(&conn, &id)?;
    Ok(Some(OperationDetail { transaction: tx, items }))
}

#[tauri::command]
pub fn operations_upsert(
    db: State<'_, DbState>,
    input: TransactionInput,
) -> AppResult<OperationDetail> {
    if !is_valid_type(&input.tx_type) {
        return Err(AppError::InvalidArg(format!(
            "invalid transaction type: {}",
            input.tx_type
        )));
    }
    if input.amount_minor < 0 {
        return Err(AppError::InvalidArg("amount must be >= 0".into()));
    }
    let now = now_ms();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut conn = db.lock();
    let prev = transactions::get(&conn, &id)?;

    let new_tx = Transaction {
        id: id.clone(),
        household_id: input.household_id.clone(),
        occurred_at: input.occurred_at,
        tx_type: input.tx_type.clone(),
        amount_minor: input.amount_minor,
        currency: input.currency.clone().unwrap_or_else(|| "RUB".into()),
        account_id: input.account_id.clone(),
        to_account_id: input.to_account_id.clone(),
        category_id: input.category_id.clone(),
        merchant_id: input.merchant_id.clone(),
        note: input.note.clone(),
        created_by: input.created_by.clone().unwrap_or_else(|| "local".into()),
        created_at: prev.as_ref().map(|p| p.created_at).unwrap_or(now),
        updated_at: now,
        source_type: SRC_MANUAL.into(),
        receipt_id: None,
        is_deleted: false,
    };

    if let Some(prev) = &prev {
        if !prev.is_deleted {
            balance::revert(&conn, prev)?;
        }
    }
    balance::apply_new(&conn, &new_tx)?;
    transactions::upsert(&conn, &new_tx)?;

    let items: Vec<ReceiptItem> = input
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|it| ReceiptItem {
            id: it.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            transaction_id: id.clone(),
            name: it.name.trim().to_string(),
            price_minor: it.price_minor,
            qty: it.qty,
            unit: it.unit.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            fns_raw: None,
        })
        .collect();
    receipt_items::replace_all(&mut conn, &id, &items)?;

    let detail = OperationDetail {
        transaction: transactions::get(&conn, &id)?.ok_or_else(|| AppError::NotFound(id.clone()))?,
        items: receipt_items::list_by_tx(&conn, &id)?,
    };
    let _ = (accounts::total_balance(&conn, &input.household_id))?;
    Ok(detail)
}

#[tauri::command]
pub fn operations_delete(db: State<'_, DbState>, id: String) -> AppResult<()> {
    let conn = db.lock();
    let tx = transactions::get(&conn, &id)?
        .ok_or_else(|| AppError::NotFound(id.clone()))?;
    if !tx.is_deleted {
        balance::revert(&conn, &tx)?;
    }
    transactions::soft_delete(&conn, &id, now_ms())?;
    Ok(())
}

#[tauri::command]
pub fn operations_item_names(
    db: State<'_, DbState>,
    household_id: String,
) -> AppResult<Vec<String>> {
    let conn = db.lock();
    receipt_items::all_distinct_names(&conn, &household_id)
}

#[tauri::command]
pub fn operations_last_price(
    db: State<'_, DbState>,
    household_id: String,
    name: String,
) -> AppResult<Option<i64>> {
    let conn = db.lock();
    receipt_items::last_price_for_name(&conn, &household_id, &name)
}
