use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Response {
    #[serde(rename = "code")]
    pub code: u32,
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Value>,
}
