use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RailboardApiError {
    pub domain: ErrorDomain,
    pub message: String,
    pub error: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorDomain {
    #[serde(rename = "vendo")]
    Vendo,
    #[serde(rename = "request")]
    Request,
}
