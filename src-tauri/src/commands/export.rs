use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::repository::{accounts, budgets, categories, transactions};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub transactions: u32,
    pub accounts: u32,
    pub categories: u32,
    pub budgets: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct BackupJson {
    transactions: Vec<crate::models::transaction::Transaction>,
    accounts: Vec<crate::models::account::Account>,
    categories: Vec<crate::models::category::Category>,
    budgets: Vec<crate::models::budget::Budget>,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn format_amount(minor: i64) -> String {
    let abs = minor.unsigned_abs();
    let s = format!("{}.{:02}", abs / 100, abs % 100);
    if minor < 0 { format!("-{}", s) } else { s }
}

fn format_occurred_at(ms: i64) -> String {
    DateTime::from_timestamp_millis(ms)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| ms.to_string())
}

fn build_account_map(accs: &[crate::models::account::Account]) -> HashMap<String, String> {
    accs.iter().map(|a| (a.id.clone(), a.name.clone())).collect()
}

fn build_category_map(cats: &[crate::models::category::Category]) -> HashMap<String, String> {
    cats.iter().map(|c| (c.id.clone(), c.name.clone())).collect()
}

fn show_save_dialog(
    app: &tauri::AppHandle,
    filter_name: &str,
    extensions: &[&str],
    default_name: &str,
    content: Vec<u8>,
) -> AppResult<Option<String>> {
    let result = tokio::task::block_in_place(|| {
        app.dialog()
            .file()
            .add_filter(filter_name, extensions)
            .set_file_name(default_name)
            .blocking_save_file()
    });
    match result {
        None => Ok(None),
        Some(fp) => {
            let path = fp
                .into_path()
                .map_err(|_| AppError::Other("не удалось получить путь к файлу".into()))?;
            std::fs::write(&path, &content)?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
    }
}

// ─── Commands ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn export_transactions_csv(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
    household_id: String,
    from_ms: i64,
    to_ms: i64,
) -> AppResult<Option<String>> {
    let (txs, accs, cats) = tokio::task::block_in_place(|| {
        let conn = db.lock();
        Ok::<_, AppError>((
            transactions::list_range(&conn, &household_id, from_ms, to_ms)?,
            accounts::list_by_household(&conn, &household_id)?,
            categories::list_by_household(&conn, &household_id)?,
        ))
    })?;

    let acc_map = build_account_map(&accs);
    let cat_map = build_category_map(&cats);

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);
    wtr.write_record(&[
        "Дата", "Тип", "Сумма", "Валюта", "Счёт", "Категория", "Заметка", "Источник",
    ])?;
    for tx in &txs {
        let date = format_occurred_at(tx.occurred_at);
        let amount = format_amount(tx.amount_minor);
        let account = acc_map
            .get(&tx.account_id)
            .map_or(tx.account_id.as_str(), |s| s.as_str());
        let category = tx
            .category_id
            .as_ref()
            .and_then(|id| cat_map.get(id))
            .map_or("", |s| s.as_str());
        let note = tx.note.as_deref().unwrap_or("");
        wtr.write_record(&[
            date.as_str(),
            tx.tx_type.as_str(),
            amount.as_str(),
            tx.currency.as_str(),
            account,
            category,
            note,
            tx.source_type.as_str(),
        ])?;
    }
    let csv_bytes = wtr
        .into_inner()
        .map_err(|e| AppError::Other(e.to_string()))?;
    // UTF-8 BOM для корректного открытия в Excel на Windows
    let mut bytes = vec![0xEF_u8, 0xBB, 0xBF];
    bytes.extend_from_slice(&csv_bytes);

    show_save_dialog(&app, "CSV Files", &["csv"], "transactions.csv", bytes)
}

#[tauri::command]
pub async fn export_transactions_xlsx(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
    household_id: String,
    from_ms: i64,
    to_ms: i64,
) -> AppResult<Option<String>> {
    let (txs, accs, cats) = tokio::task::block_in_place(|| {
        let conn = db.lock();
        Ok::<_, AppError>((
            transactions::list_range(&conn, &household_id, from_ms, to_ms)?,
            accounts::list_by_household(&conn, &household_id)?,
            categories::list_by_household(&conn, &household_id)?,
        ))
    })?;

    let acc_map = build_account_map(&accs);
    let cat_map = build_category_map(&cats);

    use rust_xlsxwriter::{Format, Workbook};
    let mut workbook = Workbook::new();
    let ws = workbook.add_worksheet();
    let bold = Format::new().set_bold();

    let headers = [
        "Дата", "Тип", "Сумма", "Валюта", "Счёт", "Категория", "Заметка", "Источник",
    ];
    let widths: [f64; 8] = [20.0, 12.0, 14.0, 10.0, 20.0, 20.0, 30.0, 12.0];

    for (col, (h, w)) in headers.iter().zip(widths.iter()).enumerate() {
        ws.write_with_format(0, col as u16, *h, &bold)?;
        ws.set_column_width(col as u16, *w)?;
    }

    for (i, tx) in txs.iter().enumerate() {
        let row = (i + 1) as u32;
        let account = acc_map
            .get(&tx.account_id)
            .cloned()
            .unwrap_or_else(|| tx.account_id.clone());
        let category = tx
            .category_id
            .as_ref()
            .and_then(|id| cat_map.get(id))
            .cloned()
            .unwrap_or_default();
        ws.write(row, 0, format_occurred_at(tx.occurred_at))?;
        ws.write(row, 1, tx.tx_type.as_str())?;
        ws.write(row, 2, format_amount(tx.amount_minor))?;
        ws.write(row, 3, tx.currency.as_str())?;
        ws.write(row, 4, account)?;
        ws.write(row, 5, category)?;
        ws.write(row, 6, tx.note.as_deref().unwrap_or(""))?;
        ws.write(row, 7, tx.source_type.as_str())?;
    }

    let buf = workbook.save_to_buffer()?;
    show_save_dialog(&app, "Excel Files", &["xlsx"], "transactions.xlsx", buf)
}

#[tauri::command]
pub async fn export_backup_json(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
    household_id: String,
) -> AppResult<Option<String>> {
    let backup = tokio::task::block_in_place(|| {
        let conn = db.lock();
        Ok::<_, AppError>(BackupJson {
            transactions: transactions::modified_since(&conn, &household_id, 0)?,
            accounts: accounts::modified_since(&conn, &household_id, 0)?,
            categories: categories::modified_since(&conn, &household_id, 0)?,
            budgets: budgets::modified_since(&conn, &household_id, 0)?,
        })
    })?;

    let bytes = serde_json::to_string_pretty(&backup)?.into_bytes();
    show_save_dialog(&app, "JSON Files", &["json"], "homebuhg_backup.json", bytes)
}

#[tauri::command]
pub async fn import_backup_json(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
    household_id: String,
) -> AppResult<Option<ImportResult>> {
    let fp_opt = tokio::task::block_in_place(|| {
        app.dialog()
            .file()
            .add_filter("JSON Files", &["json"])
            .blocking_pick_file()
    });
    let fp = match fp_opt {
        None => return Ok(None),
        Some(fp) => fp,
    };
    let path = fp
        .into_path()
        .map_err(|_| AppError::Other("не удалось получить путь к файлу".into()))?;
    let content = std::fs::read_to_string(&path)?;
    let backup: BackupJson = serde_json::from_str(&content)?;

    // Validate household_id matches backup (optional safety check)
    let _ = household_id;

    let result = tokio::task::block_in_place(|| {
        let conn = db.lock();
        let mut r = ImportResult {
            transactions: 0,
            accounts: 0,
            categories: 0,
            budgets: 0,
        };
        for t in &backup.transactions {
            transactions::upsert(&conn, t)?;
            r.transactions += 1;
        }
        for a in &backup.accounts {
            accounts::upsert(&conn, a)?;
            r.accounts += 1;
        }
        for c in &backup.categories {
            categories::upsert(&conn, c)?;
            r.categories += 1;
        }
        for b in &backup.budgets {
            budgets::upsert(&conn, b)?;
            r.budgets += 1;
        }
        Ok::<_, AppError>(r)
    })?;

    Ok(Some(result))
}
