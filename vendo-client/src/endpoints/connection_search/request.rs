use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionTravelers {
    #[serde(rename = "ermaessigungen")]
    pub discounts: Vec<String>,
    #[serde(rename = "reisendenTyp")]
    pub traveler_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionSearchTravelerProfile {
    #[serde(rename = "reisende")]
    pub travelers: Vec<VendoConnectionTravelers>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionTime {
    #[serde(rename = "reiseDatum")]
    pub trip_date: DateTime<FixedOffset>,
    #[serde(rename = "zeitPunktArt")]
    pub time_type: TimeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeType {
    Departure,
    Arrival,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionWish {
    #[serde(rename = "abgangsLocationId")]
    pub departure_location_id: String,
    #[serde(rename = "verkehrsmittel")]
    pub transportation_types: Vec<VendoConnectionSearchTransportType>,
    #[serde(rename = "zeitWunsch")]
    pub desired_time: VendoConnectionTime,
    #[serde(rename = "zielLocationId")]
    pub arrival_location_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VendoConnectionSearchTransportType {
    #[serde(rename = "ALLE")]
    All,
    #[serde(rename = "HOCHGESCHWINDIGKEITSZUEGE")]
    HighspeedTrains,
    #[serde(rename = "INTERCITYUNDEUROCITYZUEGE")]
    ICAndECTrains,
    #[serde(rename = "INTERREGIOUNDSCHNELLZUEGE")]
    InterregionalAndFastTrains,
    #[serde(rename = "NAHVERKEHRSONSTIGEZUEGE")]
    RegionalAndOtherTrains,
    #[serde(rename = "SBAHNEN")]
    SuburbanTrains,
    #[serde(rename = "BUSSE")]
    Busses,
    #[serde(rename = "SCHIFFE")]
    Boats,
    #[serde(rename = "UBAHN")]
    Subway,
    #[serde(rename = "STRASSENBAHN")]
    Tram,
    #[serde(rename = "ANRUFPFLICHTIGEVERKEHRE")]
    CallRequiringTransportTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionSearchJourney {
    #[serde(rename = "wunsch")]
    pub desired_data: VendoConnectionWish,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendoConnectionSearchRequest {
    #[serde(rename = "autonomeReservierung")]
    pub autonomous_reservation: bool,
    #[serde(rename = "einstiegsTypList")]
    pub einstiegs_typ_list: Vec<String>,
    #[serde(rename = "klasse")]
    pub class: VendoConnectionSearchClass,
    #[serde(rename = "reiseHin")]
    pub outward_journey: VendoConnectionSearchJourney,
    #[serde(rename = "reisendenProfil")]
    pub traveler_profile: VendoConnectionSearchTravelerProfile,
    #[serde(rename = "reservierungsKontingenteVorhanden")]
    pub reservation_contingents_available: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum VendoConnectionSearchClass {
    #[serde(rename = "KLASSE_1")]
    First,
    #[serde(rename = "KLASSE_2")]
    Second,
}