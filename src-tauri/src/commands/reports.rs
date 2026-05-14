use crate::db::DbState;
use crate::error::AppResult;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRange {
    pub household_id: String,
    pub start_ms: i64,
    pub end_ms: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyPoint {
    pub bucket: String,
    pub income_minor: i64,
    pub expense_minor: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySpend {
    pub category_id: String,
    pub category_name: String,
    pub color: i64,
    pub spent_minor: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportSummary {
    pub total_income_minor: i64,
    pub total_expense_minor: i64,
    pub balance_minor: i64,
}

#[tauri::command]
pub fn reports_summary(db: State<'_, DbState>, range: ReportRange) -> AppResult<ReportSummary> {
    let conn = db.lock();
    let income: i64 = conn.query_row(
        "SELECT COALESCE(SUM(amount_minor), 0) FROM transactions \
         WHERE household_id = ?1 AND type = 'INCOME' AND is_deleted = 0 \
         AND occurred_at >= ?2 AND occurred_at < ?3",
        params![range.household_id, range.start_ms, range.end_ms],
        |row| row.get(0),
    )?;
    let expense: i64 = conn.query_row(
        "SELECT COALESCE(SUM(amount_minor), 0) FROM transactions \
         WHERE household_id = ?1 AND type = 'EXPENSE' AND is_deleted = 0 \
         AND occurred_at >= ?2 AND occurred_at < ?3",
        params![range.household_id, range.start_ms, range.end_ms],
        |row| row.get(0),
    )?;
    Ok(ReportSummary {
        total_income_minor: income,
        total_expense_minor: expense,
        balance_minor: income - expense,
    })
}

#[tauri::command]
pub fn reports_monthly(db: State<'_, DbState>, range: ReportRange) -> AppResult<Vec<MonthlyPoint>> {
    let conn = db.lock();
    let mut stmt = conn.prepare(
        "SELECT strftime('%Y-%m', occurred_at/1000, 'unixepoch') as bucket, \
         SUM(CASE WHEN type = 'INCOME' THEN amount_minor ELSE 0 END) as income, \
         SUM(CASE WHEN type = 'EXPENSE' THEN amount_minor ELSE 0 END) as expense \
         FROM transactions WHERE household_id = ?1 AND is_deleted = 0 \
         AND occurred_at >= ?2 AND occurred_at < ?3 \
         GROUP BY bucket ORDER BY bucket",
    )?;
    let rows = stmt.query_map(
        params![range.household_id, range.start_ms, range.end_ms],
        |row| {
            Ok(MonthlyPoint {
                bucket: row.get(0)?,
                income_minor: row.get(1)?,
                expense_minor: row.get(2)?,
            })
        },
    )?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

#[tauri::command]
pub fn reports_top_categories(
    db: State<'_, DbState>,
    range: ReportRange,
    limit: Option<i64>,
) -> AppResult<Vec<CategorySpend>> {
    let conn = db.lock();
    let mut stmt = conn.prepare(
        "SELECT c.id, c.name, c.color, COALESCE(SUM(t.amount_minor), 0) as spent \
         FROM transactions t JOIN categories c ON c.id = t.category_id \
         WHERE t.household_id = ?1 AND t.type = 'EXPENSE' AND t.is_deleted = 0 \
         AND t.occurred_at >= ?2 AND t.occurred_at < ?3 \
         GROUP BY c.id ORDER BY spent DESC LIMIT ?4",
    )?;
    let rows = stmt.query_map(
        params![range.household_id, range.start_ms, range.end_ms, limit.unwrap_or(20)],
        |row| {
            Ok(CategorySpend {
                category_id: row.get(0)?,
                category_name: row.get(1)?,
                color: row.get(2)?,
                spent_minor: row.get(3)?,
            })
        },
    )?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
