use crate::db::DbState;
use crate::domain::session;
use crate::error::AppResult;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub user_id: String,
    pub household_id: String,
}

#[tauri::command]
pub fn session_bootstrap(db: State<'_, DbState>) -> AppResult<SessionInfo> {
    let (uid, hid) = session::ensure_local_session(&db)?;
    Ok(SessionInfo {
        user_id: uid,
        household_id: hid,
    })
}
