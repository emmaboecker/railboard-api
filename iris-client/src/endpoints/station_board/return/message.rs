use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::helpers::parse_iris_date;

use self::lookup::iris_message_lookup;

mod lookup;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub timestamp: NaiveDateTime,
    #[schema(nullable)]
    /// The message code (e.G. `59` for `Schnee und Eis`)
    pub code: Option<i32>,
    /// The matched text from the message code (e.G. `Schnee und Eis` when code is `95`)
    pub matched_text: Option<String>,
    #[schema(nullable)]
    pub category: Option<String>,
    #[schema(nullable)]
    pub valid_from: Option<NaiveDateTime>,
    #[schema(nullable)]
    pub valid_to: Option<NaiveDateTime>,
    pub message_status: MessageStatus,
    #[schema(nullable)]
    pub priority: Option<MessagePriority>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, ToSchema)]
pub enum MessageStatus {
    /// A HIM message (generated through the Hafas Information Manager)c
    HafasInformationManager,
    /// A message about a quality change
    QualityChange,
    ///  A free text message
    Free,
    /// A message about the cause of a delay
    CauseOfDelay,
    /// An IBIS message (generated from IRIS-AP)
    Ibis,
    /// An IBIS message (generated from IRIS-AP) not yet assigned to a train
    UnassignedIbis,
    /// A major disruption
    Disruption,
    /// A connection
    Connection,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MessagePriority {
    High,
    Medium,
    Low,
    Done,
}

impl From<crate::station_board::response::Message> for Message {
    fn from(value: crate::station_board::response::Message) -> Self {
        Self {
            id: value.id,
            timestamp: parse_iris_date(&value.timestamp)
                .map(|timestamp| timestamp.naive_local())
                .unwrap(),
            code: value.code,
            matched_text: value.code.as_ref().and_then(iris_message_lookup),
            category: value.category,
            valid_from: value.valid_from.and_then(|valid_from| {
                parse_iris_date(&valid_from).map(|valid_from| valid_from.naive_local())
            }),
            valid_to: value.valid_to.and_then(|valid_to| {
                parse_iris_date(&valid_to).map(|valid_to| valid_to.naive_local())
            }),
            message_status: value.message_status.into(),
            priority: value.priority.map(|priority| priority.into()),
        }
    }
}

impl From<crate::station_board::response::MessageStatus> for MessageStatus {
    fn from(value: crate::station_board::response::MessageStatus) -> Self {
        match value {
            crate::station_board::response::MessageStatus::HafasInformationManager => {
                MessageStatus::HafasInformationManager
            }
            crate::station_board::response::MessageStatus::QualityChange => {
                MessageStatus::QualityChange
            }
            crate::station_board::response::MessageStatus::Free => MessageStatus::Free,
            crate::station_board::response::MessageStatus::CauseOfDelay => {
                MessageStatus::CauseOfDelay
            }
            crate::station_board::response::MessageStatus::Ibis => MessageStatus::Ibis,
            crate::station_board::response::MessageStatus::UnassignedIbis => {
                MessageStatus::UnassignedIbis
            }
            crate::station_board::response::MessageStatus::Disruption => MessageStatus::Disruption,
            crate::station_board::response::MessageStatus::Connection => MessageStatus::Connection,
        }
    }
}

impl From<crate::station_board::response::MessagePriority> for MessagePriority {
    fn from(value: crate::station_board::response::MessagePriority) -> Self {
        match value {
            crate::station_board::response::MessagePriority::High => MessagePriority::High,
            crate::station_board::response::MessagePriority::Medium => MessagePriority::Medium,
            crate::station_board::response::MessagePriority::Low => MessagePriority::Low,
            crate::station_board::response::MessagePriority::Done => MessagePriority::Done,
        }
    }
}
