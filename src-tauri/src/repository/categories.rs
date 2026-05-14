use crate::error::AppResult;
use crate::models::category::Category;
use rusqlite::{params, Connection, Row};

fn row_to_category(row: &Row<'_>) -> rusqlite::Result<Category> {
    Ok(Category {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        name: row.get("name")?,
        category_type: row.get("type")?,
        parent_id: row.get("parent_id")?,
        color: row.get("color")?,
        icon_key: row.get("icon_key")?,
        sort_order: row.get("sort_order")?,
        updated_at: row.get("updated_at")?,
        is_deleted: row.get::<_, i64>("is_deleted")? != 0,
    })
}

pub fn list_by_household(conn: &Connection, household_id: &str) -> AppResult<Vec<Category>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM categories WHERE household_id = ?1 AND is_deleted = 0 \
         ORDER BY type, sort_order, name",
    )?;
    let rows = stmt.query_map(params![household_id], row_to_category)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<Option<Category>> {
    let mut stmt = conn.prepare("SELECT * FROM categories WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], row_to_category)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn upsert(conn: &Connection, c: &Category) -> AppResult<()> {
    conn.execute(
        "INSERT INTO categories \
        (id, household_id, name, type, parent_id, color, icon_key, sort_order, updated_at, is_deleted) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, name=excluded.name, type=excluded.type, \
         parent_id=excluded.parent_id, color=excluded.color, icon_key=excluded.icon_key, \
         sort_order=excluded.sort_order, updated_at=excluded.updated_at, is_deleted=excluded.is_deleted",
        params![
            c.id,
            c.household_id,
            c.name,
            c.category_type,
            c.parent_id,
            c.color,
            c.icon_key,
            c.sort_order,
            c.updated_at,
            c.is_deleted as i64,
        ],
    )?;
    Ok(())
}

pub fn soft_delete(conn: &Connection, id: &str, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE categories SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

pub fn modified_since(
    conn: &Connection,
    household_id: &str,
    since: i64,
) -> AppResult<Vec<Category>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM categories WHERE household_id = ?1 AND updated_at > ?2",
    )?;
    let rows = stmt.query_map(params![household_id, since], row_to_category)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
