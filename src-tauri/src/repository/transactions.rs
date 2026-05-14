use crate::error::AppResult;
use crate::models::transaction::Transaction;
use rusqlite::{params, Connection, Row};

fn row_to_tx(row: &Row<'_>) -> rusqlite::Result<Transaction> {
    Ok(Transaction {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        occurred_at: row.get("occurred_at")?,
        tx_type: row.get("type")?,
        amount_minor: row.get("amount_minor")?,
        currency: row.get("currency")?,
        account_id: row.get("account_id")?,
        to_account_id: row.get("to_account_id")?,
        category_id: row.get("category_id")?,
        merchant_id: row.get("merchant_id")?,
        note: row.get("note")?,
        created_by: row.get("created_by")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        source_type: row.get("source_type")?,
        receipt_id: row.get("receipt_id")?,
        is_deleted: row.get::<_, i64>("is_deleted")? != 0,
    })
}

pub fn list_by_household(
    conn: &Connection,
    household_id: &str,
    limit: i64,
    offset: i64,
) -> AppResult<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM transactions WHERE household_id = ?1 AND is_deleted = 0 \
         ORDER BY occurred_at DESC, created_at DESC LIMIT ?2 OFFSET ?3",
    )?;
    let rows = stmt.query_map(params![household_id, limit, offset], row_to_tx)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<Option<Transaction>> {
    let mut stmt = conn.prepare("SELECT * FROM transactions WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], row_to_tx)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn upsert(conn: &Connection, t: &Transaction) -> AppResult<()> {
    conn.execute(
        "INSERT INTO transactions \
        (id, household_id, occurred_at, type, amount_minor, currency, account_id, to_account_id, \
         category_id, merchant_id, note, created_by, created_at, updated_at, source_type, \
         receipt_id, is_deleted) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, occurred_at=excluded.occurred_at, type=excluded.type, \
         amount_minor=excluded.amount_minor, currency=excluded.currency, account_id=excluded.account_id, \
         to_account_id=excluded.to_account_id, category_id=excluded.category_id, \
         merchant_id=excluded.merchant_id, note=excluded.note, created_by=excluded.created_by, \
         created_at=excluded.created_at, updated_at=excluded.updated_at, source_type=excluded.source_type, \
         receipt_id=excluded.receipt_id, is_deleted=excluded.is_deleted",
        params![
            t.id,
            t.household_id,
            t.occurred_at,
            t.tx_type,
            t.amount_minor,
            t.currency,
            t.account_id,
            t.to_account_id,
            t.category_id,
            t.merchant_id,
            t.note,
            t.created_by,
            t.created_at,
            t.updated_at,
            t.source_type,
            t.receipt_id,
            t.is_deleted as i64,
        ],
    )?;
    Ok(())
}

pub fn soft_delete(conn: &Connection, id: &str, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE transactions SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

pub fn modified_since(
    conn: &Connection,
    household_id: &str,
    since: i64,
) -> AppResult<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM transactions WHERE household_id = ?1 AND updated_at > ?2",
    )?;
    let rows = stmt.query_map(params![household_id, since], row_to_tx)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
