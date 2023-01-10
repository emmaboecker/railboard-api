use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationSearchRequest {
    pub search_term: String,
    pub location_types: Vec<String>,
}
