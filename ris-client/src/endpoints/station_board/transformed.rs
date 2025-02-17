use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoard {
    pub eva: String,
    pub name: String,
    pub time_start: DateTime<FixedOffset>,
    pub time_end: DateTime<FixedOffset>,
    pub items: Vec<RisStationBoardItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoardItem {
    pub journey_id: String,

    pub station_eva: String,
    pub station_name: String,

    pub category: String,
    pub train_type: String,
    pub train_number: u32,
    pub line_indicator: String,

    pub cancelled: bool,

    pub arrival: Option<DepartureArrival>,
    pub departure: Option<DepartureArrival>,

    #[schema(nullable)]
    pub platform_scheduled: Option<String>,
    #[schema(nullable)]
    pub platform_realtime: Option<String>,

    pub origin_eva: String,
    pub origin_name: String,
    pub destination_eva: String,
    pub destination_name: String,

    pub administation: RisStationBoardItemAdministration,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoardItemAdministration {
    pub id: String,
    pub operator_code: String,
    pub operator_name: String,
    pub ris_operator_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepartureArrival {
    /// Since ris returns dates with seconds it also rounds up this number if the seconds are 50 for example
    pub delay: i32,
    pub time_scheduled: DateTime<FixedOffset>,
    pub time_realtime: DateTime<FixedOffset>,
    pub time_type: String,
}
