use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsResponse {
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub origin_schedule: JourneyDetailsStation,
    pub destination_schedule: JourneyDetailsStation,
    pub r#type: String,
    pub journey_canceled: bool,
    #[serde(default)]
    pub events: Vec<JourneyDetailsEvent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsStation {
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsEvent {
    pub station: JourneyDetailsStation,
    pub passenger_change: bool,
    pub on_demand: bool,
    pub time_schedule: DateTime<FixedOffset>,
    pub time_type: String,
    pub time: Option<DateTime<FixedOffset>>,
    pub platform_schedule: Option<String>,
    pub platform: Option<String>,
    pub messages: Vec<JourneyDetailsMessage>,
    pub disruptions: Vec<JourneyDetailsDisruption>,
    pub r#type: EventType,
    #[serde(rename = "arrivalOrDepartureID")]
    pub arrival_or_departure_id: String,
    pub additional: bool,
    pub canceled: bool,
    pub administration: Administration,
    pub transport: Transport,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Arrival,
    Departure,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsMessage {
    pub code: Option<String>,
    pub r#type: String,
    pub display_priority: Option<i32>,
    pub category: Option<String>,
    pub text: String,
    pub text_short: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsDisruption {
    #[serde(rename = "disruptionID")]
    pub disruption_id: String,
    #[serde(rename = "disruptionCommunicationID")]
    pub disruption_communication_id: Option<String>,
    pub display_priority: i32,
    pub descriptions: JourneyDetailsDisruptionDescriptions,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct JourneyDetailsDisruptionDescriptions {
    pub de: JourneyDetailsDisruptionDescription,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetailsDisruptionDescription {
    pub text: String,
    pub text_short: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Administration {
    #[serde(rename = "administrationID")]
    pub administration_id: String,
    pub operator_code: String,
    pub operator_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub r#type: String,
    pub category: String,
    pub number: i32,
    pub line: Option<String>,
    pub label: Option<String>,
    pub replacement_transport: Option<ReplacementTransport>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementTransport {
    pub real_type: String,
}
