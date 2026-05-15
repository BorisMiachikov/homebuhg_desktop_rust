use serde::Serialize;
use tauri::State;
use tauri_plugin_store::StoreExt;

use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::repository::now_ms;
use crate::sync::{auth, sync_service};

const STORE_FILE: &str = "sync_config.json";

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus {
    pub logged_in: bool,
    pub last_sync_ms: i64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    pub uploaded: u32,
    pub downloaded: u32,
}

fn store_str(app: &tauri::AppHandle, key: &str) -> String {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_str().map(|s| s.to_owned()))
        .unwrap_or_default()
}

fn store_i64(app: &tauri::AppHandle, key: &str) -> i64 {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
}

#[tauri::command]
pub async fn sync_login(
    app: tauri::AppHandle,
    email: String,
    password: String,
    project_id: String,
    api_key: String,
    household_id: String,
) -> AppResult<SyncStatus> {
    let client = reqwest::Client::new();
    let tokens = auth::sign_in(&client, &api_key, &email, &password).await?;

    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Other(e.to_string()))?;
    store.set("project_id", project_id.as_str());
    store.set("api_key", api_key.as_str());
    store.set("id_token", tokens.id_token.as_str());
    store.set("refresh_token", tokens.refresh_token.as_str());
    store.set("local_id", tokens.local_id.as_str());
    store.set("household_id", household_id.as_str());
    store.set("last_sync_ms", 0i64);
    store
        .save()
        .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(SyncStatus {
        logged_in: true,
        last_sync_ms: 0,
    })
}

#[tauri::command]
pub async fn sync_status(app: tauri::AppHandle) -> AppResult<SyncStatus> {
    let refresh_token = store_str(&app, "refresh_token");
    let last_sync_ms = store_i64(&app, "last_sync_ms");
    Ok(SyncStatus {
        logged_in: !refresh_token.is_empty(),
        last_sync_ms,
    })
}

#[tauri::command]
pub async fn sync_now(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
) -> AppResult<SyncResult> {
    let refresh_token = store_str(&app, "refresh_token");
    if refresh_token.is_empty() {
        return Err(AppError::Unauthorized("not logged in".into()));
    }

    let mut config = sync_service::FullSyncConfig {
        project_id: store_str(&app, "project_id"),
        api_key: store_str(&app, "api_key"),
        id_token: store_str(&app, "id_token"),
        refresh_token,
        household_id: store_str(&app, "household_id"),
        last_sync_ms: store_i64(&app, "last_sync_ms"),
    };

    let client = reqwest::Client::new();
    let stats = sync_service::run_sync(&client, &mut config, &db).await?;

    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Other(e.to_string()))?;
    store.set("id_token", config.id_token.as_str());
    store.set("refresh_token", config.refresh_token.as_str());
    store.set("last_sync_ms", now_ms());
    store
        .save()
        .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(SyncResult {
        uploaded: stats.uploaded,
        downloaded: stats.downloaded,
    })
}

#[tauri::command]
pub async fn sync_logout(app: tauri::AppHandle) -> AppResult<()> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Other(e.to_string()))?;
    store.set("id_token", "");
    store.set("refresh_token", "");
    store.set("local_id", "");
    store.set("last_sync_ms", 0i64);
    store
        .save()
        .map_err(|e| AppError::Other(e.to_string()))?;
    Ok(())
}
