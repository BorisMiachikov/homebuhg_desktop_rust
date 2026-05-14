use crate::error::AppResult;
use crate::models::account::Account;
use rusqlite::{params, Connection, Row};

fn row_to_account(row: &Row<'_>) -> rusqlite::Result<Account> {
    Ok(Account {
        id: row.get("id")?,
        household_id: row.get("household_id")?,
        name: row.get("name")?,
        account_type: row.get("type")?,
        currency: row.get("currency")?,
        balance_minor: row.get("balance_minor")?,
        credit_limit_minor: row.get("credit_limit_minor")?,
        grace_period_days: row.get("grace_period_days")?,
        payment_due_day: row.get("payment_due_day")?,
        color: row.get("color")?,
        icon_key: row.get("icon_key")?,
        is_archived: row.get::<_, i64>("is_archived")? != 0,
        updated_at: row.get("updated_at")?,
    })
}

pub fn list_by_household(conn: &Connection, household_id: &str) -> AppResult<Vec<Account>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM accounts WHERE household_id = ?1 AND is_archived = 0 ORDER BY name",
    )?;
    let rows = stmt.query_map(params![household_id], row_to_account)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<Option<Account>> {
    let mut stmt = conn.prepare("SELECT * FROM accounts WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], row_to_account)?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn upsert(conn: &Connection, a: &Account) -> AppResult<()> {
    conn.execute(
        "INSERT INTO accounts \
        (id, household_id, name, type, currency, balance_minor, credit_limit_minor, \
         grace_period_days, payment_due_day, color, icon_key, is_archived, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13) \
         ON CONFLICT(id) DO UPDATE SET \
         household_id=excluded.household_id, name=excluded.name, type=excluded.type, \
         currency=excluded.currency, balance_minor=excluded.balance_minor, \
         credit_limit_minor=excluded.credit_limit_minor, grace_period_days=excluded.grace_period_days, \
         payment_due_day=excluded.payment_due_day, color=excluded.color, icon_key=excluded.icon_key, \
         is_archived=excluded.is_archived, updated_at=excluded.updated_at",
        params![
            a.id,
            a.household_id,
            a.name,
            a.account_type,
            a.currency,
            a.balance_minor,
            a.credit_limit_minor,
            a.grace_period_days,
            a.payment_due_day,
            a.color,
            a.icon_key,
            a.is_archived as i64,
            a.updated_at,
        ],
    )?;
    Ok(())
}

pub fn set_archived(conn: &Connection, id: &str, archived: bool, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE accounts SET is_archived = ?1, updated_at = ?2 WHERE id = ?3",
        params![archived as i64, now, id],
    )?;
    Ok(())
}

pub fn adjust_balance(conn: &Connection, id: &str, delta_minor: i64, now: i64) -> AppResult<()> {
    conn.execute(
        "UPDATE accounts SET balance_minor = balance_minor + ?1, updated_at = ?2 WHERE id = ?3",
        params![delta_minor, now, id],
    )?;
    Ok(())
}

pub fn total_balance(conn: &Connection, household_id: &str) -> AppResult<i64> {
    let v: i64 = conn.query_row(
        "SELECT COALESCE(SUM(balance_minor), 0) FROM accounts \
         WHERE household_id = ?1 AND is_archived = 0",
        params![household_id],
        |row| row.get(0),
    )?;
    Ok(v)
}

pub fn modified_since(
    conn: &Connection,
    household_id: &str,
    since: i64,
) -> AppResult<Vec<Account>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM accounts WHERE household_id = ?1 AND updated_at > ?2",
    )?;
    let rows = stmt.query_map(params![household_id, since], row_to_account)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
