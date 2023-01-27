use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardResponse {
    pub is_arrival: bool,
    pub eva_no: String,
    pub station_name: String,
    pub time_start: DateTime<FixedOffset>,
    pub time_end: DateTime<FixedOffset>,
    pub items: Vec<StationBoardItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItem {
    pub station: StationBoardItemStation,
    pub train: StationBoardItemVehicle,
    pub category: String,
    pub platform: String,
    pub platform_predicted: String,
    pub time_predicted: DateTime<FixedOffset>,
    pub time: DateTime<FixedOffset>,
    pub time_type: String,
    pub canceled: bool,
    pub diff: u32,
    pub origin: Option<StationBoardItemStation>,
    pub destination: Option<StationBoardItemStation>,
    pub administration: StationBoardItemAdministration,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItemStation {
    pub eva_no: String,
    pub name: String,
    pub canceled: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItemVehicle {
    pub journey_id: String,
    pub line_name: String,
    pub no: u32,
    pub category: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItemAdministration {
    pub id: String,
    pub operator_code: String,
    pub operator_name: String,
}
