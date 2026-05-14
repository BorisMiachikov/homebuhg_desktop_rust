pub mod migrations;

use crate::error::AppResult;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub struct DbState {
    conn: Mutex<Connection>,
}

impl DbState {
    pub fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("db poisoned")
    }
}

pub fn init(app: &tauri::AppHandle) -> AppResult<DbState> {
    let dir = app
        .path()
        .app_data_dir()
        .expect("no app data dir");
    std::fs::create_dir_all(&dir)?;
    let path: PathBuf = dir.join("homebuhg.sqlite");
    let mut conn = Connection::open(&path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    migrations::run(&mut conn)?;
    Ok(DbState {
        conn: Mutex::new(conn),
    })
}
