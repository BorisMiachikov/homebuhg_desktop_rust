use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::budget::{is_valid_period, Budget};
use crate::repository::{budgets, now_ms};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetInput {
    pub id: Option<String>,
    pub household_id: String,
    pub category_id: String,
    pub period: String,
    pub limit_minor: i64,
    pub currency: Option<String>,
    pub start_date: i64,
    pub is_rolling: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetProgress {
    pub budget: Budget,
    pub spent_minor: i64,
    pub period_start: i64,
    pub period_end: i64,
}

#[tauri::command]
pub fn budgets_list(db: State<'_, DbState>, household_id: String) -> AppResult<Vec<BudgetProgress>> {
    let conn = db.lock();
    let list = budgets::list_by_household(&conn, &household_id)?;
    let now = now_ms();
    let mut out = Vec::new();
    for b in list {
        let (start, end) = period_bounds(&b, now);
        let spent: i64 = conn.query_row(
            "SELECT COALESCE(SUM(amount_minor), 0) FROM transactions \
             WHERE household_id = ?1 AND category_id = ?2 AND type = 'EXPENSE' \
             AND is_deleted = 0 AND occurred_at >= ?3 AND occurred_at < ?4",
            params![b.household_id, b.category_id, start, end],
            |row| row.get(0),
        )?;
        out.push(BudgetProgress {
            budget: b,
            spent_minor: spent,
            period_start: start,
            period_end: end,
        });
    }
    Ok(out)
}

#[tauri::command]
pub fn budgets_upsert(db: State<'_, DbState>, input: BudgetInput) -> AppResult<Budget> {
    if !is_valid_period(&input.period) {
        return Err(AppError::InvalidArg(format!("invalid period: {}", input.period)));
    }
    let now = now_ms();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let b = Budget {
        id: id.clone(),
        household_id: input.household_id,
        category_id: input.category_id,
        period: input.period,
        limit_minor: input.limit_minor,
        currency: input.currency.unwrap_or_else(|| "RUB".into()),
        start_date: input.start_date,
        is_rolling: input.is_rolling.unwrap_or(false),
        updated_at: now,
        is_deleted: false,
    };
    let conn = db.lock();
    budgets::upsert(&conn, &b)?;
    budgets::get(&conn, &id)?.ok_or_else(|| AppError::NotFound(id))
}

#[tauri::command]
pub fn budgets_delete(db: State<'_, DbState>, id: String) -> AppResult<()> {
    let conn = db.lock();
    budgets::soft_delete(&conn, &id, now_ms())
}

fn period_bounds(b: &Budget, now: i64) -> (i64, i64) {
    use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
    let now_dt = Utc.timestamp_millis_opt(now).single().unwrap_or_else(Utc::now);
    let start_dt: DateTime<Utc> = Utc
        .timestamp_millis_opt(b.start_date)
        .single()
        .unwrap_or(now_dt);
    let period_ms = match b.period.as_str() {
        "WEEK" => 7 * 24 * 3600 * 1000,
        "MONTH" => 30 * 24 * 3600 * 1000,
        "YEAR" => 365 * 24 * 3600 * 1000,
        _ => 30 * 24 * 3600 * 1000,
    };
    if b.is_rolling {
        let start = now - period_ms;
        return (start, now);
    }
    match b.period.as_str() {
        "WEEK" => {
            let weekday = now_dt.weekday().num_days_from_monday() as i64;
            let start = now_dt - Duration::days(weekday);
            let start_ms = start
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .map(|d| Utc.from_utc_datetime(&d).timestamp_millis())
                .unwrap_or(now);
            (start_ms, start_ms + period_ms)
        }
        "MONTH" => {
            let start = now_dt
                .date_naive()
                .with_day(1)
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|d| Utc.from_utc_datetime(&d).timestamp_millis())
                .unwrap_or(now);
            let end = chrono::NaiveDate::from_ymd_opt(
                now_dt.year() + if now_dt.month() == 12 { 1 } else { 0 },
                if now_dt.month() == 12 { 1 } else { now_dt.month() + 1 },
                1,
            )
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .map(|d| Utc.from_utc_datetime(&d).timestamp_millis())
            .unwrap_or(start + period_ms);
            (start, end)
        }
        "YEAR" => {
            let start = chrono::NaiveDate::from_ymd_opt(now_dt.year(), 1, 1)
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|d| Utc.from_utc_datetime(&d).timestamp_millis())
                .unwrap_or(start_dt.timestamp_millis());
            let end = chrono::NaiveDate::from_ymd_opt(now_dt.year() + 1, 1, 1)
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|d| Utc.from_utc_datetime(&d).timestamp_millis())
                .unwrap_or(start + period_ms);
            (start, end)
        }
        _ => (now - period_ms, now),
    }
}
