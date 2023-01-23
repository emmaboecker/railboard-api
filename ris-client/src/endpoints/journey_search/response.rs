use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ToSchema)]
pub struct RisJourneySearchResponse {
    pub journeys: Vec<RisJourneySearchElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ToSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneySearchSchedule {
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneySearchTransport {
    pub r#type: String,
    pub category: String,
    pub number: i32,
    #[schema(nullable)]
    pub line: Option<String>,
    #[schema(nullable)]
    pub label: Option<String>,
    #[schema(nullable)]
    pub replacement_transport: Option<String>,
}
