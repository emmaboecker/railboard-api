use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStop {
    pub id: String,
    pub station_eva: String,
    pub station_name: String,
    pub messages: Vec<Message>,
    pub departure: Option<StationBoardStopDeparture>,
    pub arrival: Option<StationBoardStopArrival>,
    pub planned_platform: Option<String>,
    pub real_platform: Option<String>,
    pub cancelled: bool,
    pub added: bool,
    pub hidden: bool,
    pub train_type: String,
    pub train_number: String,
    pub line_indicator: String,
    pub route: Vec<RouteStop>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct RouteStop {
    pub name: String,
    pub cancelled: Option<bool>,
    pub added: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStopArrival {
    pub planned_time: DateTime<FixedOffset>,
    pub real_time: Option<DateTime<FixedOffset>>,
    pub wings: Vec<String>,
    pub origin: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardStopDeparture {
    pub planned_time: DateTime<FixedOffset>,
    pub real_time: Option<DateTime<FixedOffset>>,
    pub wings: Vec<String>,
    pub direction: String,
}
