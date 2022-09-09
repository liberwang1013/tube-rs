use crate::enums::StatusCode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub schema: String,
    pub code: StatusCode,
    pub body: serde_json::Value,
}
