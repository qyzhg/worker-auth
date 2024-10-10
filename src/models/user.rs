use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct User {
    #[serde(rename = "id")]
    pub(crate) id: Option<usize>,
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "email")]
    pub(crate) email: String,
    #[serde(rename = "password")]
    pub(crate) password: String,
    #[serde(rename = "created_at")]
    pub(crate) created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub(crate) updated_at: Option<String>,
    #[serde(rename = "is_deleted", skip_serializing)]
    pub is_deleted: Option<usize>,
}
