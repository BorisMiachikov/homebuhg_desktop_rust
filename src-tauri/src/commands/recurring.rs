use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::recurring::RecurringRule;
use crate::repository::{now_ms, recurring};
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringInput {
    pub id: Option<String>,
    pub household_id: String,
    pub template_json: String,
    pub rrule: String,
    pub next_run_at: i64,
    pub is_active: Option<bool>,
}

#[tauri::command]
pub fn recurring_list(db: State<'_, DbState>, household_id: String) -> AppResult<Vec<RecurringRule>> {
    let conn = db.lock();
    recurring::list_by_household(&conn, &household_id)
}

#[tauri::command]
pub fn recurring_upsert(db: State<'_, DbState>, input: RecurringInput) -> AppResult<RecurringRule> {
    let now = now_ms();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let r = RecurringRule {
        id: id.clone(),
        household_id: input.household_id,
        template_json: input.template_json,
        rrule: input.rrule,
        next_run_at: input.next_run_at,
        last_run_at: None,
        is_active: input.is_active.unwrap_or(true),
        updated_at: now,
    };
    let conn = db.lock();
    recurring::upsert(&conn, &r)?;
    recurring::get(&conn, &id)?.ok_or_else(|| AppError::NotFound(id))
}

#[tauri::command]
pub fn recurring_delete(db: State<'_, DbState>, id: String) -> AppResult<()> {
    let conn = db.lock();
    recurring::delete(&conn, &id)
}
