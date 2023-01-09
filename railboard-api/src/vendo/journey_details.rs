use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use vendo_client::journey_details::{
    JourneyDetailsAttribute, JourneyDetailsError, JourneyDetailsHimNotice,
};

use crate::{
    error::{ErrorDomain, RailboardApiError, RailboardResult},
    types::{Attribute, HimNotice, Time},
    VENDO_CLIENT,
};

pub async fn journey_details(Path(id): Path<String>) -> RailboardResult<Json<JoruneyDetails>> {
    let response = VENDO_CLIENT.journey_details(id).await?;

    let mapped = JoruneyDetails {
        short_name: response.short_name,
        name: response.name,
        long_name: response.long_name,
        destination: response.destination,

        stops: response
            .stops
            .into_iter()
            .map(|stop| Stop {
                name: stop.name,
                arrival: stop.arrival.map(|arrival| Time {
                    scheduled: arrival,
                    realtime: stop.realtime_arrival,
                }),
                departure: stop.departure.map(|arrival| Time {
                    scheduled: arrival,
                    realtime: stop.realtime_arrival,
                }),
                platform: stop.platform,
                realtime_platform: stop.realtime_platform,
                notes: stop.notes.into_iter().map(|note| note.text).collect(),
                him_notices: stop
                    .him_notices
                    .into_iter()
                    .map(|from| from.into())
                    .collect(),
                attributes: stop
                    .attributes
                    .into_iter()
                    .map(|from| from.into())
                    .collect(),
            })
            .collect(),

        transport_number: response.transport_number,
        product_type: response.product_type,
        notes: response.notes.into_iter().map(|note| note.text).collect(),
        him_notices: response
            .him_notices
            .into_iter()
            .map(|from| from.into())
            .collect(),
        attributes: response
            .attributes
            .into_iter()
            .map(|from| from.into())
            .collect(),
        schedule: TrainSchedule {
            regular_schedule: response.schedule.regular_schedule,
            days_of_operation: response.schedule.days_of_operation,
        },
        journey_day: response.journey_day,
    };

    Ok(Json(mapped))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoruneyDetails {
    pub short_name: String,
    pub name: String,
    pub long_name: String,
    pub destination: String,

    pub stops: Vec<Stop>,

    pub transport_number: Option<String>,
    pub product_type: String,

    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,

    pub schedule: TrainSchedule,
    pub journey_day: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainSchedule {
    pub regular_schedule: String,
    pub days_of_operation: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub name: String,
    pub arrival: Option<Time>,
    pub departure: Option<Time>,
    pub platform: Option<String>,
    pub realtime_platform: Option<String>,
    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,
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

impl From<JourneyDetailsError> for RailboardApiError {
    fn from(value: JourneyDetailsError) -> Self {
        match value {
            JourneyDetailsError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get journey details from Vendo: {}", err),
                error: None,
            },
            JourneyDetailsError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get journey details from Vendo: {}", err),
                error: Some(serde_json::to_value(err).unwrap()),
            },
        }
    }
}
