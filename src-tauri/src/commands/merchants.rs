use crate::db::DbState;
use crate::error::AppResult;
use crate::models::merchant::Merchant;
use crate::repository::merchants;
use tauri::State;

#[tauri::command]
pub fn merchants_list(db: State<'_, DbState>, household_id: String) -> AppResult<Vec<Merchant>> {
    let conn = db.lock();
    merchants::list_by_household(&conn, &household_id)
}
