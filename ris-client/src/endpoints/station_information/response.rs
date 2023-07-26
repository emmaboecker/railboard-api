use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StationInformationResponse {
    #[serde(rename = "stopPlaces")]
    pub stations: Vec<StationInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StationInformation {
    pub eva_number: String,
    #[serde(rename = "stationID")]
    pub station_id: Option<String>,
    pub names: Translatable<StationNameContent>,
    pub metropolis: Option<Translatable<String>>,
    pub available_transports: Vec<String>,
    pub transport_associations: Vec<String>,
    pub country_code: String,
    pub state: String,
    pub municipality_key: String,
    pub time_zone: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct Translatable<T> {
    pub de: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationNameContent {
    pub name_long: String,
    pub speech_long: Option<String>,
    pub speech_short: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
}
