use crate::error::AppResult;
use crate::models::recurring::RecurringRule;
use rusqlite::{params, Connection, Row};

fn row_to_rule(row: &Row<'_>) -> rusqlite::Result<RecurringRule> {
    Ok(RecurringRule {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        template_json: row.get("template_json")?,
        rrule: row.get("rrule")?,
        next_run_at: row.get("next_run_at")?,
        last_run_at: row.get("last_run_at")?,
        is_active: row.get::<_, i64>("is_active")? != 0,
        updated_at: row.get("updated_at")?,
    })
}

pub fn list_by_household(conn: &Connection, household_id: &str) -> AppResult<Vec<RecurringRule>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM recurring_rules WHERE household_id = ?1 ORDER BY next_run_at",
    )?;
    let rows = stmt.query_map(params![household_id], row_to_rule)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn list_due(conn: &Connection, now: i64) -> AppResult<Vec<RecurringRule>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM recurring_rules WHERE is_active = 1 AND next_run_at <= ?1",
    )?;
    let rows = stmt.query_map(params![now], row_to_rule)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<Option<RecurringRule>> {
    let mut stmt = conn.prepare("SELECT * FROM recurring_rules WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], row_to_rule)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn upsert(conn: &Connection, r: &RecurringRule) -> AppResult<()> {
    conn.execute(
        "INSERT INTO recurring_rules \
        (id, household_id, template_json, rrule, next_run_at, last_run_at, is_active, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, template_json=excluded.template_json, rrule=excluded.rrule, \
         next_run_at=excluded.next_run_at, last_run_at=excluded.last_run_at, \
         is_active=excluded.is_active, updated_at=excluded.updated_at",
        params![
            r.id,
            r.household_id,
            r.template_json,
            r.rrule,
            r.next_run_at,
            r.last_run_at,
            r.is_active as i64,
            r.updated_at,
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    conn.execute(
        "DELETE FROM recurring_rules WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}
