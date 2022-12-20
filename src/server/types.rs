use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub scheduled: DateTime<FixedOffset>,
    pub realtime: Option<DateTime<FixedOffset>>,
}
