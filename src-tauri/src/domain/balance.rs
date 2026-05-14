use crate::error::{AppError, AppResult};
use crate::models::transaction::{Transaction, TYPE_EXPENSE, TYPE_INCOME, TYPE_TRANSFER};
use crate::repository::{accounts, now_ms};
use rusqlite::Connection;

pub fn apply(conn: &Connection, t: &Transaction, sign: i64) -> AppResult<()> {
    let now = now_ms();
    let amount = t.amount_minor * sign;
    match t.tx_type.as_str() {
        TYPE_INCOME => {
            accounts::adjust_balance(conn, &t.account_id, amount, now)?;
        }
        TYPE_EXPENSE => {
            accounts::adjust_balance(conn, &t.account_id, -amount, now)?;
        }
        TYPE_TRANSFER => {
            let to = t
                .to_account_id
                .as_ref()
                .ok_or_else(|| AppError::InvalidArg("transfer needs toAccountId".into()))?;
            accounts::adjust_balance(conn, &t.account_id, -amount, now)?;
            accounts::adjust_balance(conn, to, amount, now)?;
        }
        other => return Err(AppError::InvalidArg(format!("unknown type: {}", other))),
    }
    Ok(())
}

pub fn revert(conn: &Connection, t: &Transaction) -> AppResult<()> {
    apply(conn, t, -1)
}

pub fn apply_new(conn: &Connection, t: &Transaction) -> AppResult<()> {
    apply(conn, t, 1)
}
