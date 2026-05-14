use crate::error::AppResult;
use crate::models::merchant::Merchant;
use rusqlite::{params, Connection, Row};

fn row_to_merchant(row: &Row<'_>) -> rusqlite::Result<Merchant> {
    Ok(Merchant {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        name: row.get("name")?,
        default_category_id: row.get("default_category_id")?,
        last_used_at: row.get("last_used_at")?,
        updated_at: row.get("updated_at")?,
        is_deleted: row.get::<_, i64>("is_deleted")? != 0,
    })
}

pub fn list_by_household(conn: &Connection, household_id: &str) -> AppResult<Vec<Merchant>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM merchants WHERE household_id = ?1 AND is_deleted = 0 \
         ORDER BY last_used_at DESC",
    )?;
    let rows = stmt.query_map(params![household_id], row_to_merchant)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn upsert(conn: &Connection, m: &Merchant) -> AppResult<()> {
    conn.execute(
        "INSERT INTO merchants \
        (id, household_id, name, default_category_id, last_used_at, updated_at, is_deleted) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, name=excluded.name, \
         default_category_id=excluded.default_category_id, last_used_at=excluded.last_used_at, \
         updated_at=excluded.updated_at, is_deleted=excluded.is_deleted",
        params![
            m.id,
            m.household_id,
            m.name,
            m.default_category_id,
            m.last_used_at,
            m.updated_at,
            m.is_deleted as i64,
        ],
    )?;
    Ok(())
}

pub fn soft_delete(conn: &Connection, id: &str, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE merchants SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}
