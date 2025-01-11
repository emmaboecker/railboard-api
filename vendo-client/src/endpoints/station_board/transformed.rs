use crate::shared::Time;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VendoStationBoard {
    pub day: String,
    pub time: String,
    pub id: String,
    pub station_board: Vec<StationBoardElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardElement {
    pub journey_id: String,
    #[schema(nullable)]
    pub arrival: Option<StationBoardArrival>,
    #[schema(nullable)]
    pub departure: Option<StationBoardDeparture>,
    pub product_type: String,
    pub short_name: String,
    pub name: String,
    #[schema(nullable)]
    pub scheduled_platform: Option<String>,
    #[schema(nullable)]
    pub realtime_platform: Option<String>,
    pub notes: Vec<String>,
    pub request_station: StationBoardRequestedStation,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardArrival {
    pub origin: String,
    pub time: Time,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardDeparture {
    pub destination: String,
    pub time: Time,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardRequestedStation {
    pub eva: String,
    pub name: String,
    pub location_id: String,
}
