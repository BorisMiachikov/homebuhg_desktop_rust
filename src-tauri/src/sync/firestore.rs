use serde_json::{Map, Value, json};

use crate::error::{AppError, AppResult};

pub struct SyncConfig {
    pub project_id: String,
    pub id_token: String,
}

pub struct WriteOp {
    pub name: String,
    pub fields: Map<String, Value>,
}

const BATCH_SIZE: usize = 400;

pub fn doc_name(
    project_id: &str,
    household_id: &str,
    collection: &str,
    doc_id: &str,
) -> String {
    format!(
        "projects/{}/databases/(default)/documents/households/{}/{}/{}",
        project_id, household_id, collection, doc_id
    )
}

pub async fn run_query(
    client: &reqwest::Client,
    config: &SyncConfig,
    household_id: &str,
    collection_id: &str,
    updated_after_ms: i64,
) -> AppResult<Vec<Map<String, Value>>> {
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/households/{}:runQuery",
        config.project_id, household_id
    );
    let body = json!({
        "structuredQuery": {
            "from": [{"collectionId": collection_id}],
            "where": {
                "fieldFilter": {
                    "field": {"fieldPath": "updatedAt"},
                    "op": "GREATER_THAN",
                    "value": {"integerValue": updated_after_ms.to_string()}
                }
            }
        }
    });
    let resp = client
        .post(&url)
        .bearer_auth(&config.id_token)
        .json(&body)
        .send()
        .await?;
    let status = resp.status();
    if status.as_u16() == 401 {
        return Err(AppError::Unauthorized("token expired".into()));
    }
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(AppError::Firebase(extract_error(&text)));
    }
    let array: Vec<Value> = serde_json::from_str(&text)?;
    let mut result = Vec::new();
    for item in array {
        if let Some(doc) = item.get("document") {
            if let Some(fields) = doc.get("fields").and_then(|f| f.as_object()) {
                result.push(fields.clone());
            }
        }
    }
    Ok(result)
}

pub async fn batch_write(
    client: &reqwest::Client,
    config: &SyncConfig,
    writes: Vec<WriteOp>,
) -> AppResult<()> {
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:batchWrite",
        config.project_id
    );
    for chunk in writes.chunks(BATCH_SIZE) {
        let writes_json: Vec<Value> = chunk
            .iter()
            .map(|op| {
                json!({
                    "update": {
                        "name": op.name,
                        "fields": op.fields
                    }
                })
            })
            .collect();
        let body = json!({"writes": writes_json});
        let resp = client
            .post(&url)
            .bearer_auth(&config.id_token)
            .json(&body)
            .send()
            .await?;
        let status = resp.status();
        if status.as_u16() == 401 {
            return Err(AppError::Unauthorized("token expired".into()));
        }
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(AppError::Firebase(extract_error(&text)));
        }
    }
    Ok(())
}

fn extract_error(body: &str) -> String {
    if let Ok(v) = serde_json::from_str::<Value>(body) {
        if let Some(msg) = v["error"]["message"].as_str() {
            return msg.to_owned();
        }
    }
    body.to_owned()
}
