use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

// Arrivals
#[derive(Debug, Serialize, Deserialize)]
pub struct StationBoardArrivalsResponse {
    #[serde(rename = "bahnhofstafelAnkunftPositionen")]
    pub arrivals: Vec<StationBoardArrivalsElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoardArrivalsElement {
    #[serde(rename = "zuglaufId")]
    pub id: String,
    #[serde(rename = "kurztext")]
    pub short_name: String,
    #[serde(rename = "mitteltext")]
    pub name: String,
    #[serde(rename = "abfrageOrt")]
    pub requested_station: StationBoardRequestedStation,
    #[serde(rename = "abgangsOrt")]
    pub origin: StationBoardArrivalsElementOrigin,
    #[serde(rename = "ankunftsDatum")]
    pub arrival_date: DateTime<FixedOffset>,
    #[serde(rename = "ezAnkunftsDatum")]
    pub realtime_arrival_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "gleis")]
    pub platform: Option<String>,
    #[serde(rename = "ezGleis")]
    pub realtime_platform: Option<String>,
    #[serde(rename = "echtzeitNotizen")]
    pub notes: Vec<Note>,
    #[serde(rename = "produktGattung")]
    pub product_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoardArrivalsElementOrigin {
    pub name: String,
    #[serde(rename = "locationId")]
    pub location_id: String,
}

// Departures

#[derive(Debug, Serialize, Deserialize)]
pub struct StationBoardDeparturesResponse {
    #[serde(rename = "bahnhofstafelAbfahrtPositionen")]
    pub departures: Vec<StationBoardDeparturesElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoardDeparturesElement {
    #[serde(rename = "zuglaufId")]
    pub id: String,
    #[serde(rename = "kurztext")]
    pub short_name: String,
    #[serde(rename = "mitteltext")]
    pub name: String,
    #[serde(rename = "abfrageOrt")]
    pub requested_station: StationBoardRequestedStation,
    #[serde(rename = "richtung")]
    pub destination_name: String,

    #[serde(rename = "abgangsDatum")]
    pub departure_date: DateTime<FixedOffset>,
    #[serde(rename = "ezAbgangsDatum")]
    pub realtime_departure_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "gleis")]
    pub platform: Option<String>,
    #[serde(rename = "ezGleis")]
    pub realtime_platform: Option<String>,
    #[serde(rename = "echtzeitNotizen")]
    pub notes: Vec<Note>,
    #[serde(rename = "produktGattung")]
    pub product_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StationBoardRequestedStation {
    pub name: String,
    #[serde(rename = "locationId")]
    pub location_id: String,
    #[serde(rename = "evaNr")]
    pub eva: String,
}
