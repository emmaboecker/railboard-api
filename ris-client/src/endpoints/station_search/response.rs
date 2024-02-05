use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::station_information::RisPosition;


#[derive(Deserialize, Debug, Serialize, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisStationSearchResponse {
    pub stop_places: Vec<RisStationSearchElement>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisStationSearchElement {
    pub eva_number: String,
    #[serde(rename = "stationID")]
    #[schema(nullable)]
    pub station_id: Option<String>,
    pub group_members: Vec<String>,
    pub names: RisStationSearchTranslatable,
    pub available_transports: Vec<String>,
    pub position: RisPosition,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct RisStationSearchTranslatable {
    pub de: RisStationSearchNameContent,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisStationSearchNameContent {
    pub name_long: String,
    #[schema(nullable)]
    pub speech_long: Option<String>,
    #[schema(nullable)]
    pub speech_short: Option<String>,
    #[schema(nullable)]
    pub symbol: Option<String>,
}
