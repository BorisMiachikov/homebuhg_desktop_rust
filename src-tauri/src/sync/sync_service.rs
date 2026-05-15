use crate::db::DbState;
use crate::error::AppResult;
use crate::repository::{accounts, budgets, categories, transactions};
use crate::sync::{auth, firestore, mapper};

pub struct FullSyncConfig {
    pub project_id: String,
    pub api_key: String,
    pub id_token: String,
    pub refresh_token: String,
    pub household_id: String,
    pub last_sync_ms: i64,
}

pub struct SyncStats {
    pub uploaded: u32,
    pub downloaded: u32,
}

pub async fn run_sync(
    client: &reqwest::Client,
    config: &mut FullSyncConfig,
    db: &DbState,
) -> AppResult<SyncStats> {
    let new_tokens =
        auth::refresh_token(client, &config.api_key, &config.refresh_token).await?;
    config.id_token = new_tokens.id_token;
    config.refresh_token = new_tokens.refresh_token;

    let fs_config = firestore::SyncConfig {
        project_id: config.project_id.clone(),
        id_token: config.id_token.clone(),
    };

    let uploaded = upload(client, &fs_config, db, &config.household_id, config.last_sync_ms).await?;
    let downloaded =
        download(client, &fs_config, db, &config.household_id, config.last_sync_ms).await?;

    Ok(SyncStats { uploaded, downloaded })
}

async fn upload(
    client: &reqwest::Client,
    config: &firestore::SyncConfig,
    db: &DbState,
    household_id: &str,
    since_ms: i64,
) -> AppResult<u32> {
    let hid = household_id.to_owned();
    let pid = config.project_id.clone();
    let mut count = 0u32;

    // transactions
    {
        let hid2 = hid.clone();
        let txs = tokio::task::block_in_place(|| {
            let conn = db.lock();
            transactions::modified_since(&conn, &hid2, since_ms)
        })?;
        if !txs.is_empty() {
            let writes = txs
                .iter()
                .map(|t| firestore::WriteOp {
                    name: firestore::doc_name(&pid, &hid, "transactions", &t.id),
                    fields: mapper::transaction_to_fields(t),
                })
                .collect();
            firestore::batch_write(client, config, writes).await?;
            count += txs.len() as u32;
        }
    }

    // accounts
    {
        let hid2 = hid.clone();
        let items = tokio::task::block_in_place(|| {
            let conn = db.lock();
            accounts::modified_since(&conn, &hid2, since_ms)
        })?;
        if !items.is_empty() {
            let writes = items
                .iter()
                .map(|a| firestore::WriteOp {
                    name: firestore::doc_name(&pid, &hid, "accounts", &a.id),
                    fields: mapper::account_to_fields(a),
                })
                .collect();
            firestore::batch_write(client, config, writes).await?;
            count += items.len() as u32;
        }
    }

    // categories
    {
        let hid2 = hid.clone();
        let items = tokio::task::block_in_place(|| {
            let conn = db.lock();
            categories::modified_since(&conn, &hid2, since_ms)
        })?;
        if !items.is_empty() {
            let writes = items
                .iter()
                .map(|c| firestore::WriteOp {
                    name: firestore::doc_name(&pid, &hid, "categories", &c.id),
                    fields: mapper::category_to_fields(c),
                })
                .collect();
            firestore::batch_write(client, config, writes).await?;
            count += items.len() as u32;
        }
    }

    // budgets
    {
        let hid2 = hid.clone();
        let items = tokio::task::block_in_place(|| {
            let conn = db.lock();
            budgets::modified_since(&conn, &hid2, since_ms)
        })?;
        if !items.is_empty() {
            let writes = items
                .iter()
                .map(|b| firestore::WriteOp {
                    name: firestore::doc_name(&pid, &hid, "budgets", &b.id),
                    fields: mapper::budget_to_fields(b),
                })
                .collect();
            firestore::batch_write(client, config, writes).await?;
            count += items.len() as u32;
        }
    }

    Ok(count)
}

async fn download(
    client: &reqwest::Client,
    config: &firestore::SyncConfig,
    db: &DbState,
    household_id: &str,
    since_ms: i64,
) -> AppResult<u32> {
    let mut count = 0u32;

    // transactions
    {
        let docs = firestore::run_query(client, config, household_id, "transactions", since_ms).await?;
        for fields in &docs {
            match mapper::fields_to_transaction(fields) {
                Ok(t) => {
                    tokio::task::block_in_place(|| {
                        let conn = db.lock();
                        transactions::upsert(&conn, &t)
                    })?;
                    count += 1;
                }
                Err(e) => tracing::warn!("skip bad transaction doc: {}", e),
            }
        }
    }

    // accounts
    {
        let docs = firestore::run_query(client, config, household_id, "accounts", since_ms).await?;
        for fields in &docs {
            match mapper::fields_to_account(fields) {
                Ok(a) => {
                    tokio::task::block_in_place(|| {
                        let conn = db.lock();
                        accounts::upsert(&conn, &a)
                    })?;
                    count += 1;
                }
                Err(e) => tracing::warn!("skip bad account doc: {}", e),
            }
        }
    }

    // categories
    {
        let docs = firestore::run_query(client, config, household_id, "categories", since_ms).await?;
        for fields in &docs {
            match mapper::fields_to_category(fields) {
                Ok(c) => {
                    tokio::task::block_in_place(|| {
                        let conn = db.lock();
                        categories::upsert(&conn, &c)
                    })?;
                    count += 1;
                }
                Err(e) => tracing::warn!("skip bad category doc: {}", e),
            }
        }
    }

    // budgets
    {
        let docs = firestore::run_query(client, config, household_id, "budgets", since_ms).await?;
        for fields in &docs {
            match mapper::fields_to_budget(fields) {
                Ok(b) => {
                    tokio::task::block_in_place(|| {
                        let conn = db.lock();
                        budgets::upsert(&conn, &b)
                    })?;
                    count += 1;
                }
                Err(e) => tracing::warn!("skip bad budget doc: {}", e),
            }
        }
    }

    Ok(count)
}
