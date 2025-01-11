use crate::journey_details::response::{JourneyDetailsAttribute, JourneyDetailsHimNotice};
use crate::shared::{Attribute, HimNotice, Time};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VendoJourneyDetails {
    pub short_name: String,
    pub name: String,
    pub long_name: Option<String>,
    pub destination: String,

    pub journey_id: String,

    pub stops: Vec<VendoStop>,

    #[schema(nullable)]
    pub transport_number: Option<String>,
    pub product_type: String,

    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,

    pub schedule: VendoTrainSchedule,
    pub journey_day: String,

    #[schema(nullable)]
    pub polyline: Option<Vec<PolylinePosition>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolylinePosition {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VendoTrainSchedule {
    pub regular_schedule: String,
    #[schema(nullable)]
    pub days_of_operation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VendoStop {
    pub name: String,
    pub eva: String,
    pub position: PolylinePosition,
    #[schema(nullable)]
    pub arrival: Option<Time>,
    #[schema(nullable)]
    pub departure: Option<Time>,
    #[schema(nullable)]
    pub platform: Option<String>,
    #[schema(nullable)]
    pub realtime_platform: Option<String>,
    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,
    #[schema(nullable)]
    pub service_note: Option<Attribute>,
}

impl From<JourneyDetailsHimNotice> for HimNotice {
    fn from(notice: JourneyDetailsHimNotice) -> Self {
        HimNotice {
            text: notice.text,
            heading: notice.heading,
            priority: notice.priority,
        }
    }
}

impl From<JourneyDetailsAttribute> for Attribute {
    fn from(attribute: JourneyDetailsAttribute) -> Self {
        Attribute {
            text: attribute.text,
            key: attribute.key,
        }
    }
}
