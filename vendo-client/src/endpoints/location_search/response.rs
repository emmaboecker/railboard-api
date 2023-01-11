use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationSearchResult {
    pub coordinates: LocationSearchCoordinates,
    pub eva_nr: String,
    pub location_id: String,
    pub location_type: String,
    pub name: String,
    pub products: Vec<String>,
    pub weight: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationSearchCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}
