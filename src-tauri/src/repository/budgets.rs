use crate::error::AppResult;
use crate::models::budget::Budget;
use rusqlite::{params, Connection, Row};

fn row_to_budget(row: &Row<'_>) -> rusqlite::Result<Budget> {
    Ok(Budget {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        category_id: row.get("category_id")?,
        period: row.get("period")?,
        limit_minor: row.get("limit_minor")?,
        currency: row.get("currency")?,
        start_date: row.get("start_date")?,
        is_rolling: row.get::<_, i64>("is_rolling")? != 0,
        updated_at: row.get("updated_at")?,
        is_deleted: row.get::<_, i64>("is_deleted")? != 0,
    })
}

pub fn list_by_household(conn: &Connection, household_id: &str) -> AppResult<Vec<Budget>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM budgets WHERE household_id = ?1 AND is_deleted = 0 ORDER BY start_date DESC",
    )?;
    let rows = stmt.query_map(params![household_id], row_to_budget)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<Option<Budget>> {
    let mut stmt = conn.prepare("SELECT * FROM budgets WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], row_to_budget)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn upsert(conn: &Connection, b: &Budget) -> AppResult<()> {
    conn.execute(
        "INSERT INTO budgets \
        (id, household_id, category_id, period, limit_minor, currency, start_date, is_rolling, \
         updated_at, is_deleted) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, category_id=excluded.category_id, period=excluded.period, \
         limit_minor=excluded.limit_minor, currency=excluded.currency, start_date=excluded.start_date, \
         is_rolling=excluded.is_rolling, updated_at=excluded.updated_at, is_deleted=excluded.is_deleted",
        params![
            b.id,
            b.household_id,
            b.category_id,
            b.period,
            b.limit_minor,
            b.currency,
            b.start_date,
            b.is_rolling as i64,
            b.updated_at,
            b.is_deleted as i64,
        ],
    )?;
    Ok(())
}

pub fn soft_delete(conn: &Connection, id: &str, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE budgets SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

pub fn modified_since(
    conn: &Connection,
    household_id: &str,
    since: i64,
) -> AppResult<Vec<Budget>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM budgets WHERE household_id = ?1 AND updated_at > ?2",
    )?;
    let rows = stmt.query_map(params![household_id, since], row_to_budget)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
