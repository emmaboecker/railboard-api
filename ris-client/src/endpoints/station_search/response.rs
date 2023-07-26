use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::station_information::{Position, StationNameContent, Translatable};

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisStationSearchResponse {
    pub stop_places: Vec<StationSearchResponse>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationSearchResponse {
    pub eva_number: String,
    #[serde(rename = "stationID")]
    #[schema(nullable)]
    pub station_id: Option<String>,
    pub group_members: Vec<String>,
    pub names: Translatable<StationNameContent>,
    pub available_transports: Vec<String>,
    pub position: Position,
}
