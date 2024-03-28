use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemWing {
    pub category: String,
    pub destination: RisStationBoardItemStopAtStopPlace,
    #[serde(rename = "differingDestination")]
    pub differing_destination: Option<RisStationBoardItemStopAtStopPlace>,
    pub direction: Option<RisStationBoardItemDirectionInfo>,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub label: Option<String>,
    pub line: String,
    pub number: i32,
    #[serde(rename = "replacementTransport")]
    pub replacement_transport: RisStationBoardItemReplacementTransport,
    #[serde(rename = "separationAt")]
    pub separation_at: RisStationBoardItemStop,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemViaStop {
    pub additional: bool,
    pub canceled: bool,
    #[serde(rename = "displayPriority")]
    pub display_priority: i64,
    #[serde(rename = "evaNumber")]
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepartureTransport {
    pub category: String,
    pub destination: RisStationBoardItemStopAtStopPlace,
    #[serde(rename = "differingDestination")]
    pub differing_destination: Option<RisStationBoardItemStopAtStopPlace>,
    pub direction: RisStationBoardItemDirectionInfo,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub label: Option<String>,
    pub line: Option<String>,
    pub number: i32,
    #[serde(rename = "replacementTransport")]
    pub replacement_transport: RisStationBoardItemReplacementTransport,
    #[serde(rename = "type")]
    pub r#type: RisStationBoardTransportType,
    pub via: Vec<RisStationBoardItemViaStop>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrivalTransport {
    pub category: String,
    pub origin: RisStationBoardItemStopAtStopPlace,
    #[serde(rename = "differingOrigin")]
    pub differing_origin: Option<RisStationBoardItemStopAtStopPlace>,
    pub direction: RisStationBoardItemDirectionInfo,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub label: Option<String>,
    pub line: Option<String>,
    pub number: i32,
    #[serde(rename = "replacementTransport")]
    pub replacement_transport: RisStationBoardItemReplacementTransport,
    #[serde(rename = "type")]
    pub r#type: RisStationBoardTransportType,
    pub via: Vec<RisStationBoardItemViaStop>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardMessage {
    pub category: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "displayPriority")]
    pub display_priority: Option<i32>,
    pub text: String,
    #[serde(rename = "textShort")]
    pub text_short: Option<String>,
    #[serde(rename = "type")]
    pub r#type: RisStationBoardMessageType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RisStationBoardMessageType {
    CustomerText,
    QualityVariation,
    CustomerReason,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardDisruptionDescription {
    pub text: String,
    #[serde(rename = "textShort")]
    pub text_short: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardDisruptionDescriptionTranslatable {
    #[serde(rename = "DE")]
    pub de: RisStationBoardDisruptionDescription,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardDisruption {
    pub descriptions: RisStationBoardDisruptionDescriptionTranslatable,
    #[serde(rename = "displayPriority")]
    pub display_priority: Option<i32>,
    #[serde(rename = "disruptionCommunicationID")]
    pub disruption_communication_id: Option<String>,
    #[serde(rename = "disruptionID")]
    pub disruption_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemReplacementTransport {
    #[serde(rename = "realType")]
    pub real_type: RisStationBoardTransportType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemStop {
    #[serde(rename = "evaNumber")]
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemDirectionInfo {
    #[serde(rename = "stopPlaces")]
    pub stop_places: Vec<RisStationBoardItemStop>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemStopAtStopPlace {
    pub canceled: bool,
    #[serde(rename = "evaNumber")]
    pub eva_number: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemReplacement {
    pub category: String,
    pub destination: RisStationBoardItemStopAtStopPlace,
    #[serde(rename = "differingDestination")]
    pub differing_destination: Option<RisStationBoardItemStopAtStopPlace>,
    pub direction: Option<RisStationBoardItemDirectionInfo>,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    pub label: Option<String>,
    pub line: Option<String>,
    pub number: i32,
    #[serde(rename = "replacementTransport")]
    pub replacement_transport: Option<RisStationBoardItemReplacementTransport>,
    #[serde(rename = "type")]
    pub r#type: RisStationBoardTransportType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardItemAttribute {
    pub code: String,
    pub text: String,
    #[serde(rename = "displayPriority")]
    pub display_priority: Option<i32>,
    #[serde(rename = "displayPriorityDetail")]
    pub display_priority_detail: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisAdministration {
    #[serde(rename = "administrationID")]
    pub administration_id: String,
    #[serde(rename = "operatorCode")]
    pub operator_code: String,
    #[serde(rename = "operatorName")]
    pub operator_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardArrivalItem {
    pub additional: bool,
    pub administration: RisAdministration,
    pub attributes: Vec<RisStationBoardItemAttribute>,
    pub canceled: bool,
    #[serde(rename = "continuationBy")]
    pub continuation_by: RisStationBoardItemReplacement,
    #[serde(rename = "arrivalID")]
    pub arrival_id: String,
    pub disruptions: Vec<RisStationBoardDisruption>,
    #[serde(rename = "pastDisruptions")]
    pub past_disruptions: bool,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    #[serde(rename = "journeyType")]
    pub journey_type: RisStationBoardItemJourneyType,
    #[serde(default)]
    pub messages: Vec<RisStationBoardMessage>,
    #[serde(rename = "onDemand")]
    pub on_demand: bool,
    pub platform: String,
    #[serde(rename = "platformSchedule")]
    pub platform_schedule: Option<String>,
    #[serde(rename = "reliefBy")]
    pub relief_by: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "reliefFor")]
    pub relief_for: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "replacedBy")]
    pub replaced_by: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "replacementFor")]
    pub replacement_for: Vec<RisStationBoardItemReplacement>,
    pub station: RisStationBoardItemStop,
    pub time: DateTime<FixedOffset>,
    #[serde(rename = "timeSchedule")]
    pub time_schedule: DateTime<FixedOffset>,
    #[serde(rename = "timeType")]
    pub time_type: String,
    pub transport: DepartureTransport,
    #[serde(rename = "travelsWith")]
    pub travels_with: Vec<RisStationBoardItemWing>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardDepartureItem {
    pub additional: bool,
    pub administration: RisAdministration,
    pub attributes: Vec<RisStationBoardItemAttribute>,
    pub canceled: bool,
    #[serde(rename = "continuationBy")]
    pub continuation_by: RisStationBoardItemReplacement,
    #[serde(rename = "departureID")]
    pub departure_id: String,
    pub disruptions: Vec<RisStationBoardDisruption>,
    #[serde(rename = "futureDisruptions")]
    pub future_disruptions: bool,
    #[serde(rename = "journeyID")]
    pub journey_id: String,
    #[serde(rename = "journeyType")]
    pub journey_type: RisStationBoardItemJourneyType,
    #[serde(default)]
    pub messages: Vec<RisStationBoardMessage>,
    #[serde(rename = "onDemand")]
    pub on_demand: bool,
    pub platform: String,
    #[serde(rename = "platformSchedule")]
    pub platform_schedule: Option<String>,
    #[serde(rename = "reliefBy")]
    pub relief_by: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "reliefFor")]
    pub relief_for: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "replacedBy")]
    pub replaced_by: Vec<RisStationBoardItemReplacement>,
    #[serde(rename = "replacementFor")]
    pub replacement_for: Vec<RisStationBoardItemReplacement>,
    pub station: RisStationBoardItemStop,
    pub time: DateTime<FixedOffset>,
    #[serde(rename = "timeSchedule")]
    pub time_schedule: DateTime<FixedOffset>,
    #[serde(rename = "timeType")]
    pub time_type: String,
    pub transport: ArrivalTransport,
    #[serde(rename = "travelsWith")]
    pub travels_with: Vec<RisStationBoardItemWing>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RisStationBoardTimeType {
    Schedule,
    Preview,
    Realtime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardDeparturesResponse {
    pub departures: Vec<RisStationBoardDepartureItem>,
    pub disruptions: Vec<RisStationBoardDisruption>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RisStationBoardArrivalsResponse {
    pub arrivals: Vec<RisStationBoardArrivalItem>,
    pub disruptions: Vec<RisStationBoardDisruption>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RisStationBoardTransportType {
    HighSpeedTrain,
    IntercityTrain,
    InterregionalTrain,
    RegionalTrain,
    CityTrain,
    Bus,
    Tram,
    Ferry,
    Subway,
    Shuttle,
    Unknown,
    Scooter,
    Flight,
    Taxi,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum RisStationBoardItemJourneyType {
    Regular,
    Relief,
    Replacement,
    Extra
}