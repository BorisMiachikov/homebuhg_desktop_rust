use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::category::{is_valid_type, Category};
use crate::repository::{categories, now_ms};
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryInput {
    pub id: Option<String>,
    pub household_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: String,
    pub parent_id: Option<String>,
    pub color: Option<i64>,
    pub icon_key: Option<String>,
    pub sort_order: Option<i32>,
}

#[tauri::command]
pub fn categories_list(db: State<'_, DbState>, household_id: String) -> AppResult<Vec<Category>> {
    let conn = db.lock();
    categories::list_by_household(&conn, &household_id)
}

#[tauri::command]
pub fn categories_upsert(db: State<'_, DbState>, input: CategoryInput) -> AppResult<Category> {
    if !is_valid_type(&input.category_type) {
        return Err(AppError::InvalidArg(format!(
            "invalid category type: {}",
            input.category_type
        )));
    }
    let now = now_ms();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let c = Category {
        id: id.clone(),
        household_id: input.household_id,
        name: input.name.trim().to_string(),
        category_type: input.category_type,
        parent_id: input.parent_id,
        color: input.color.unwrap_or(0xFF9E9E9E),
        icon_key: input.icon_key.unwrap_or_else(|| "more_horiz".into()),
        sort_order: input.sort_order.unwrap_or(0),
        updated_at: now,
        is_deleted: false,
    };
    let conn = db.lock();
    categories::upsert(&conn, &c)?;
    categories::get(&conn, &id)?.ok_or_else(|| AppError::NotFound(id))
}

#[tauri::command]
pub fn categories_delete(db: State<'_, DbState>, id: String) -> AppResult<()> {
    let conn = db.lock();
    categories::soft_delete(&conn, &id, now_ms())
}
