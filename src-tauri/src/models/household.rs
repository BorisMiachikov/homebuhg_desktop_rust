use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: String,
    pub display_name: String,
    pub email: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Household {
    pub id: String,
    pub name: String,
    pub owner_uid: String,
    pub base_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMember {
    pub household_id: String,
    pub user_uid: String,
    pub role: String,
    pub joined_at: i64,
}

pub const ROLE_OWNER: &str = "OWNER";
pub const ROLE_MEMBER: &str = "MEMBER";
