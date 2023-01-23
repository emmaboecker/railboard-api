use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RisJourneySearchResponse {
    pub journeys: Vec<RisJourneySearchElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneySearchElement {
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub date: String,
    #[serde(rename = "administrationID")]
    pub administration_id: String,
    pub origin_schedule: RisJourneySearchSchedule,
    pub destination_schedule: RisJourneySearchSchedule,
    pub transport: RisJourneySearchTransport,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneySearchSchedule {
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneySearchTransport {
    pub r#type: String,
    pub category: String,
    pub number: i32,
    pub line: Option<String>,
    pub label: Option<String>,
    pub replacement_transport: Option<String>,
}
