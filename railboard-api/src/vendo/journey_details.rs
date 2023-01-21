use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use vendo_client::journey_details::{JourneyDetailsAttribute, JourneyDetailsHimNotice};

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
    types::{Attribute, HimNotice, Time},
};

use super::VendoState;

#[utoipa::path(
    get,
    path = "/vendo/v1/journey_details/{id}",
    params(("id" = String, Path, description = "The Vendo-ID of the Journey you want to get details for")),
    tag = "Vendo",
    responses(
        (status = 200, description = "The requested Journey Details", body = JourneyDetails),
        (status = 400, description = "The Error returned by Vendo", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails", body = RailboardApiError)
    )
)]
pub async fn journey_details(
    Path(id): Path<String>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<JourneyDetails>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("vendo.journey-details.{}", &id))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.vendo_client.journey_details(&id).await?;

    let mapped = JourneyDetails {
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

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JourneyDetails {
    pub short_name: String,
    pub name: String,
    pub long_name: String,
    pub destination: String,

    pub journey_id: String,

    pub stops: Vec<Stop>,

    #[schema(nullable)]
    pub transport_number: Option<String>,
    pub product_type: String,

    pub notes: Vec<String>,
    pub him_notices: Vec<HimNotice>,
    pub attributes: Vec<Attribute>,

    pub schedule: TrainSchedule,
    pub journey_day: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrainSchedule {
    pub regular_schedule: String,
    #[schema(nullable)]
    pub days_of_operation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub name: String,
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
