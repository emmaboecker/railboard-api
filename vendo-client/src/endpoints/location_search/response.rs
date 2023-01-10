use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationSearchResult {
    coordinates: LocationSearchCoordinates,
    eva_nr: String,
    location_id: String,
    location_type: String,
    name: String,
    products: Vec<String>,
    weight: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSearchCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}
