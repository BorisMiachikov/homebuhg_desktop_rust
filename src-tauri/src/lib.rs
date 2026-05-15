pub mod commands;
pub mod db;
pub mod domain;
pub mod error;
pub mod models;
pub mod repository;
pub mod sync;

use crate::commands::{accounts, budgets, categories, export as export_cmd, merchants, operations, recurring, reports, session, sync as sync_cmd};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let state = db::init(app.handle())?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            session::session_bootstrap,
            accounts::accounts_list,
            accounts::accounts_get,
            accounts::accounts_upsert,
            accounts::accounts_archive,
            accounts::accounts_total,
            categories::categories_list,
            categories::categories_upsert,
            categories::categories_delete,
            operations::operations_list,
            operations::operations_get,
            operations::operations_upsert,
            operations::operations_delete,
            operations::operations_item_names,
            operations::operations_last_price,
            budgets::budgets_list,
            budgets::budgets_upsert,
            budgets::budgets_delete,
            recurring::recurring_list,
            recurring::recurring_upsert,
            recurring::recurring_delete,
            merchants::merchants_list,
            reports::reports_summary,
            reports::reports_monthly,
            reports::reports_top_categories,
            sync_cmd::sync_login,
            sync_cmd::sync_now,
            sync_cmd::sync_status,
            sync_cmd::sync_logout,
            export_cmd::export_transactions_csv,
            export_cmd::export_transactions_xlsx,
            export_cmd::export_backup_json,
            export_cmd::import_backup_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
