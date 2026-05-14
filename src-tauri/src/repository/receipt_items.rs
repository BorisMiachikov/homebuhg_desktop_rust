use crate::error::AppResult;
use crate::models::receipt_item::ReceiptItem;
use rusqlite::{params, Connection, Row};

fn row_to_item(row: &Row<'_>) -> rusqlite::Result<ReceiptItem> {
    Ok(ReceiptItem {
        id: row.get("id")?,
        transaction_id: row.get("transaction_id")?,
        name: row.get("name")?,
        price_minor: row.get("price_minor")?,
        qty: row.get("qty")?,
        unit: row.get("unit")?,
        fns_raw: row.get("fns_raw")?,
    })
}

pub fn list_by_tx(conn: &Connection, tx_id: &str) -> AppResult<Vec<ReceiptItem>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM receipt_items WHERE transaction_id = ?1 ORDER BY rowid",
    )?;
    let rows = stmt.query_map(params![tx_id], row_to_item)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn upsert(conn: &Connection, i: &ReceiptItem) -> AppResult<()> {
    conn.execute(
        "INSERT INTO receipt_items (id, transaction_id, name, price_minor, qty, unit, fns_raw) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) \
         ON CONFLICT(id) DO UPDATE SET \
         transaction_id=excluded.transaction_id, name=excluded.name, price_minor=excluded.price_minor, \
         qty=excluded.qty, unit=excluded.unit, fns_raw=excluded.fns_raw",
        params![i.id, i.transaction_id, i.name, i.price_minor, i.qty, i.unit, i.fns_raw],
    )?;
    Ok(())
}

pub fn delete_by_tx(conn: &Connection, tx_id: &str) -> AppResult<()> {
    conn.execute(
        "DELETE FROM receipt_items WHERE transaction_id = ?1",
        params![tx_id],
    )?;
    Ok(())
}

pub fn replace_all(conn: &mut Connection, tx_id: &str, items: &[ReceiptItem]) -> AppResult<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "DELETE FROM receipt_items WHERE transaction_id = ?1",
        params![tx_id],
    )?;
    for i in items {
        tx.execute(
            "INSERT INTO receipt_items (id, transaction_id, name, price_minor, qty, unit, fns_raw) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![i.id, i.transaction_id, i.name, i.price_minor, i.qty, i.unit, i.fns_raw],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn all_distinct_names(conn: &Connection, household_id: &str) -> AppResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT ri.name FROM receipt_items ri \
         JOIN transactions t ON t.id = ri.transaction_id \
         WHERE t.household_id = ?1 AND t.is_deleted = 0 \
         ORDER BY ri.name",
    )?;
    let rows = stmt.query_map(params![household_id], |row| row.get::<_, String>(0))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn last_price_for_name(
    conn: &Connection,
    household_id: &str,
    name: &str,
) -> AppResult<Option<i64>> {
    let mut stmt = conn.prepare(
        "SELECT ri.price_minor FROM receipt_items ri \
         JOIN transactions t ON t.id = ri.transaction_id \
         WHERE t.household_id = ?1 AND ri.name = ?2 AND t.is_deleted = 0 \
         ORDER BY t.occurred_at DESC LIMIT 1",
    )?;
    let mut rows = stmt.query_map(params![household_id, name], |row| row.get::<_, i64>(0))?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}
