use crate::journey_details::response::{JourneyDetailsMessage, ReplacementTransport, Transport};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyDetails {
    pub id: String,
    pub journey_type: String,
    pub origin_name: String,
    pub origin_id: String,
    pub destination_name: String,
    pub destination_id: String,
    pub cancelled: bool,
    pub stops: Vec<RisJourneyStop>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyStop {
    pub stop_id: String,
    pub stop_name: String,
    #[schema(nullable)]
    pub arrival: Option<RisJourneyStopEvent>,
    #[schema(nullable)]
    pub departure: Option<RisJourneyStopEvent>,
    pub messages: Vec<RisJourneyDetailsMessage>,
    pub disruptions: Vec<RisJourneyStopDisruption>,
    pub transport: RisTransport,
    #[schema(nullable)]
    pub scheduled_platform: Option<String>,
    #[schema(nullable)]
    pub real_platform: Option<String>,
    pub administration: RisJourneyStopAdministration,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyStopEvent {
    pub cancelled: bool,
    pub additional: bool,
    pub on_demand: bool,
    pub scheduled: DateTime<FixedOffset>,
    #[schema(nullable)]
    pub realtime: Option<DateTime<FixedOffset>>,
    pub time_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyStopAdministration {
    pub id: String,
    pub name: String,
    pub operator_code: String,
    pub ris_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyStopDisruption {
    pub id: String,
    pub communication_id: Option<String>,
    pub priority: i32,
    pub text: String,
    #[schema(nullable)]
    pub text_short: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyDetailsMessage {
    pub code: Option<String>,
    pub r#type: String,
    pub display_priority: Option<i32>,
    pub category: Option<String>,
    pub text: String,
    pub text_short: Option<String>,
}

impl From<JourneyDetailsMessage> for RisJourneyDetailsMessage {
    fn from(message: JourneyDetailsMessage) -> Self {
        RisJourneyDetailsMessage {
            code: message.code,
            r#type: message.r#type,
            display_priority: message.display_priority,
            category: message.category,
            text: message.text,
            text_short: message.text_short,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisTransport {
    pub r#type: String,
    pub category: String,
    pub number: i32,
    pub line: Option<String>,
    pub label: Option<String>,
    pub replacement_transport: Option<RisReplacementTransport>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisReplacementTransport {
    pub real_type: String,
}

impl From<Transport> for RisTransport {
    fn from(transport: Transport) -> Self {
        RisTransport {
            r#type: transport.r#type,
            category: transport.category,
            number: transport.number,
            line: transport.line,
            label: transport.label,
            replacement_transport: transport
                .replacement_transport
                .map(RisReplacementTransport::from),
        }
    }
}

impl From<ReplacementTransport> for RisReplacementTransport {
    fn from(replacement_transport: ReplacementTransport) -> Self {
        RisReplacementTransport {
            real_type: replacement_transport.real_type,
        }
    }
}
