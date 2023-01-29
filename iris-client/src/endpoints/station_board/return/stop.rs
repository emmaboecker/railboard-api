use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::Message;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStop {
    pub id: String,
    pub station_eva: String,
    pub station_name: String,
    pub messages: Vec<Message>,
    #[schema(nullable)]
    pub departure: Option<StationBoardStopDeparture>,
    #[schema(nullable)]
    pub arrival: Option<StationBoardStopArrival>,
    #[schema(nullable)]
    pub planned_platform: Option<String>,
    #[schema(nullable)]
    pub real_platform: Option<String>,
    pub cancelled: bool,
    pub added: bool,
    pub hidden: bool,
    pub train_type: String,
    pub train_number: String,
    pub line_indicator: String,
    pub route: Vec<RouteStop>,
    #[schema(nullable)]
    pub replaces: Option<ReplacedTrain>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
pub struct ReplacedTrain {
    pub category: String,
    pub number: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
pub struct RouteStop {
    pub name: String,
    pub cancelled: bool,
    pub added: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStopArrival {
    pub planned_time: DateTime<FixedOffset>,
    #[schema(nullable)]
    pub real_time: Option<DateTime<FixedOffset>>,
    pub wings: Vec<String>,
    pub origin: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStopDeparture {
    pub planned_time: DateTime<FixedOffset>,
    #[schema(nullable)]
    pub real_time: Option<DateTime<FixedOffset>>,
    pub wings: Vec<String>,
    pub direction: String,
}
