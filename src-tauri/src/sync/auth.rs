use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub id_token: String,
    pub refresh_token: String,
    pub local_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInResponse {
    id_token: String,
    refresh_token: String,
    local_id: String,
}

#[derive(Deserialize)]
struct RefreshResponse {
    id_token: String,
    refresh_token: String,
}

pub async fn sign_in(
    client: &reqwest::Client,
    api_key: &str,
    email: &str,
    password: &str,
) -> AppResult<AuthTokens> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );
    let body = serde_json::json!({
        "email": email,
        "password": password,
        "returnSecureToken": true
    });
    let resp = client.post(&url).json(&body).send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(AppError::Firebase(extract_firebase_error(&text)));
    }
    let parsed: SignInResponse = serde_json::from_str(&text)?;
    Ok(AuthTokens {
        id_token: parsed.id_token,
        refresh_token: parsed.refresh_token,
        local_id: parsed.local_id,
    })
}

pub async fn refresh_token(
    client: &reqwest::Client,
    api_key: &str,
    refresh_token_str: &str,
) -> AppResult<AuthTokens> {
    let url = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        api_key
    );
    let resp = client
        .post(&url)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token_str),
        ])
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(AppError::Firebase(extract_firebase_error(&text)));
    }
    let parsed: RefreshResponse = serde_json::from_str(&text)?;
    Ok(AuthTokens {
        id_token: parsed.id_token,
        refresh_token: parsed.refresh_token,
        local_id: String::new(),
    })
}

fn extract_firebase_error(body: &str) -> String {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(msg) = v["error"]["message"].as_str() {
            return msg.to_owned();
        }
    }
    body.to_owned()
}
