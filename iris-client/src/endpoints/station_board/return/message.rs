use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub timestamp: String,
    /// The message code (e.G. `59` for "Schnee und Eis")
    pub code: Option<i32>,
    pub category: Option<String>,
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub valid_from: Option<String>,
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub valid_to: Option<String>,
    pub message_status: MessageStatus,
    pub priority: Option<MessagePriority>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum MessageStatus {
    /// A HIM message (generated through the Hafas Information Manager)
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
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
            timestamp: value.timestamp,
            code: value.code,
            category: value.category,
            valid_from: value.valid_from,
            valid_to: value.valid_to,
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
