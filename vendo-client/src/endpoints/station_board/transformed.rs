use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::shared::Time;

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
    pub(crate) time: Time,
}
