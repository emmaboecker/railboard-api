use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

impl From<crate::station_information::response::StationInformation> for RisStationInformation {
    fn from(value: crate::station_information::response::StationInformation) -> Self {
        Self {
            eva: value.eva_number,
            names: RisStationNameContent {
                name_long: value.names.de.name_long,
                speech_long: value.names.de.speech_long,
                speech_short: value.names.de.speech_short,
            },
            station_id: value.station_id,
            available_transports: value.available_transports,
            transport_associations: value.transport_associations,
            country_code: value.country_code,
            state: value.state,
            municipality_key: value.municipality_key,
            time_zone: value.time_zone,
            metropolis: value.metropolis.map(|m| m.de),
            position: RisPosition {
                longitude: value.position.longitude,
                latitude: value.position.latitude,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationInformation {
    pub eva: String,
    #[schema(nullable)]
    pub station_id: Option<String>,
    pub names: RisStationNameContent,
    pub metropolis: Option<String>,
    pub available_transports: Vec<String>,
    pub transport_associations: Vec<String>,
    pub country_code: String,
    pub state: String,
    pub municipality_key: String,
    pub time_zone: String,
    pub position: RisPosition,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationNameContent {
    pub name_long: String,
    #[schema(nullable)]
    pub speech_long: Option<String>,
    #[schema(nullable)]
    pub speech_short: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RisPosition {
    pub longitude: f64,
    pub latitude: f64,
}
