use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VendoStationBoardRequest {
    #[serde(rename = "anfragezeit")]
    pub time: String,
    #[serde(rename = "datum")]
    pub date: String,
    #[serde(rename = "ursprungsBahnhofId")]
    pub station: String,
    #[serde(rename = "verkehrsmittel")]
    pub transport_types: Vec<VendoTransportType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VendoTransportType {
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

impl VendoTransportType {
    pub fn all() -> Vec<VendoTransportType> {
        vec![
            VendoTransportType::HighspeedTrains,
            VendoTransportType::ICAndECTrains,
            VendoTransportType::InterregionalAndFastTrains,
            VendoTransportType::RegionalAndOtherTrains,
            VendoTransportType::SuburbanTrains,
            VendoTransportType::Busses,
            VendoTransportType::Boats,
            VendoTransportType::Subway,
            VendoTransportType::Tram,
            VendoTransportType::CallRequiringTransportTypes,
        ]
    }
}
