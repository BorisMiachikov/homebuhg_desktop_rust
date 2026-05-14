use crate::error::AppResult;
use crate::models::household::{Household, HouseholdMember, User};
use rusqlite::{params, Connection, Row};

fn row_to_user(row: &Row<'_>) -> rusqlite::Result<User> {
    Ok(User {
        uid: row.get("uid")?,
        display_name: row.get("display_name")?,
        email: row.get("email")?,
        photo_url: row.get("photo_url")?,
    })
}

fn row_to_household(row: &Row<'_>) -> rusqlite::Result<Household> {
    Ok(Household {
        id: row.get("id")?,
        name: row.get("name")?,
        owner_uid: row.get("owner_uid")?,
        base_currency: row.get("base_currency")?,
    })
}

pub fn upsert_user(conn: &Connection, u: &User) -> AppResult<()> {
    conn.execute(
        "INSERT INTO users (uid, display_name, email, photo_url) VALUES (?1, ?2, ?3, ?4) \
         ON CONFLICT(uid) DO UPDATE SET \
         display_name=excluded.display_name, email=excluded.email, photo_url=excluded.photo_url",
        params![u.uid, u.display_name, u.email, u.photo_url],
    )?;
    Ok(())
}

pub fn upsert_household(conn: &Connection, h: &Household) -> AppResult<()> {
    conn.execute(
        "INSERT INTO households (id, name, owner_uid, base_currency) VALUES (?1, ?2, ?3, ?4) \
         ON CONFLICT(id) DO UPDATE SET \
         name=excluded.name, owner_uid=excluded.owner_uid, base_currency=excluded.base_currency",
        params![h.id, h.name, h.owner_uid, h.base_currency],
    )?;
    Ok(())
}

pub fn upsert_member(conn: &Connection, m: &HouseholdMember) -> AppResult<()> {
    conn.execute(
        "INSERT INTO household_members (household_id, user_uid, role, joined_at) \
         VALUES (?1, ?2, ?3, ?4) \
         ON CONFLICT(household_id, user_uid) DO UPDATE SET \
         role=excluded.role, joined_at=excluded.joined_at",
        params![m.household_id, m.user_uid, m.role, m.joined_at],
    )?;
    Ok(())
}

pub fn get_user(conn: &Connection, uid: &str) -> AppResult<Option<User>> {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE uid = ?1")?;
    let mut rows = stmt.query_map(params![uid], row_to_user)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn list_households_for_user(conn: &Connection, uid: &str) -> AppResult<Vec<Household>> {
    let mut stmt = conn.prepare(
        "SELECT h.* FROM households h JOIN household_members m ON m.household_id = h.id \
         WHERE m.user_uid = ?1",
    )?;
    let rows = stmt.query_map(params![uid], row_to_household)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
