use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VendoLocationSearchResult {
    pub coordinates: VendoLocationSearchCoordinates,
    #[schema(nullable)]
    pub eva_nr: Option<String>,
    pub location_id: String,
    pub location_type: String,
    pub name: String,
    #[serde(default)]
    pub products: Vec<String>,
    #[schema(nullable)]
    pub weight: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VendoLocationSearchCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}
