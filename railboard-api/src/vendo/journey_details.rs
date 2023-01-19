use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use vendo_client::journey_details::{JourneyDetailsAttribute, JourneyDetailsHimNotice};

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
    types::{Attribute, HimNotice, Time},
};

use super::VendoState;

pub async fn journey_details(
    Path(id): Path<String>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<JoruneyDetails>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("vendo.journey-details.{}", &id))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.vendo_client.journey_details(&id).await?;

    let mapped = JoruneyDetails {
        short_name: response.short_name,
        name: response.name,
        long_name: response.long_name,
        destination: response.destination,

        journey_id: id,

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
                service_note: stop.service_note.map(|service| service.into()),
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

    {
        let cached = mapped.clone();
        tokio::spawn(async move { cached.insert_to_cache(&state.cache).await });
    }

    Ok(Json(mapped))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JoruneyDetails {
    pub short_name: String,
    pub name: String,
    pub long_name: String,
    pub destination: String,

    pub journey_id: String,

    pub stops: Vec<Stop>,

    pub transport_number: Option<String>,
    pub product_type: String,

    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,

    pub schedule: TrainSchedule,
    pub journey_day: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrainSchedule {
    pub regular_schedule: String,
    pub days_of_operation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
