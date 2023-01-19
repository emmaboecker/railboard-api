use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyDetailsResponse {
    #[serde(rename = "kurztext")]
    pub short_name: String,
    #[serde(rename = "mitteltext")]
    pub name: String,
    #[serde(rename = "langtext")]
    pub long_name: String,
    #[serde(rename = "richtung")]
    pub destination: String,

    #[serde(rename = "halte")]
    pub stops: Vec<JourneyDetailsStop>,

    #[serde(rename = "verkehrsmittelNummer")]
    pub transport_number: Option<String>,
    #[serde(rename = "produktGattung")]
    pub product_type: String,

    #[serde(rename = "echtzeitNotizen")]
    pub notes: Vec<JourneyDetailsNotice>,
    #[serde(rename = "himNotizen")]
    pub him_notices: Vec<JourneyDetailsHimNotice>,
    #[serde(rename = "attributNotizen")]
    pub attributes: Vec<JourneyDetailsAttribute>,

    #[serde(rename = "fahrplan")]
    pub schedule: JourneyDetailsTrainSchedule,
    #[serde(rename = "reisetag")]
    pub journey_day: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyDetailsTrainSchedule {
    #[serde(rename = "regulaererFahrplan")]
    pub regular_schedule: String,
    #[serde(rename = "tageOhneFahrt")]
    pub days_of_operation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyDetailsStop {
    #[serde(rename = "ort")]
    pub name: String,
    #[serde(rename = "ankunftsDatum")]
    pub arrival: Option<DateTime<FixedOffset>>,
    #[serde(rename = "ezAnkunftsDatum")]
    pub realtime_arrival: Option<DateTime<FixedOffset>>,
    #[serde(rename = "abgangsDatum")]
    pub departure: Option<DateTime<FixedOffset>>,
    #[serde(rename = "ezAbgangsDatum")]
    pub realtime_departure: Option<DateTime<FixedOffset>>,
    #[serde(rename = "gleis")]
    pub platform: Option<String>,
    #[serde(rename = "ezGleis")]
    pub realtime_platform: Option<String>,
    #[serde(rename = "echtzeitNotizen")]
    pub notes: Vec<JourneyDetailsNotice>,
    #[serde(rename = "himNotizen")]
    pub him_notices: Vec<JourneyDetailsHimNotice>,
    #[serde(rename = "serviceNotiz")]
    pub service_note: Option<JourneyDetailsAttribute>,
    #[serde(rename = "attributNotizen")]
    pub attributes: Vec<JourneyDetailsAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyDetailsNotice {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyDetailsHimNotice {
    pub text: String,
    #[serde(rename = "ueberschrift")]
    pub heading: String,
    #[serde(rename = "prio")]
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyDetailsAttribute {
    pub text: String,
    pub key: String,
}
