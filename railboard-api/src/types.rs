use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub scheduled: DateTime<FixedOffset>,
    pub realtime: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notice {
    pub text: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HimNotice {
    pub text: String,
    pub heading: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub text: String,
    pub key: String,
}
