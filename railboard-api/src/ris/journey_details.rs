use std::{collections::HashSet, sync::Arc};

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, FixedOffset};
use ris_client::journey_details::{JourneyDetailsEvent, JourneyDetailsMessage, Transport};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
};

use super::RisState;

#[utoipa::path(
    get,
    path = "/ris/v1/journey_details/{id}",
    params(
        ("id" = String, Path, description = "The id of this journey (can be optained e.G. through the journey search endpoint)")
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Journey Details", body = RisJourneyDetails),
        (status = 400, description = "The Error returned by Ris, will be the Ris Domain with UnderlyingApiError Variant 3 or 4", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn journey_details(
    Path(id): Path<String>,
    state: State<Arc<RisState>>,
) -> RailboardResult<Json<RisJourneyDetails>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.journey-details.{}", &id))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.ris_client.journey_details(&id).await?;

    let mut stops: Vec<(Option<JourneyDetailsEvent>, Option<JourneyDetailsEvent>)> = Vec::new();

    'outer: for event in response.events {
        match event.r#type {
            ris_client::journey_details::EventType::Arrival => {
                for stop in stops.iter_mut() {
                    if stop
                        .1
                        .as_ref()
                        .map(|departure| {
                            stop.0.is_none()
                                && departure.station.eva_number == event.station.eva_number
                                && departure.time_schedule >= event.time_schedule
                        })
                        .unwrap_or(false)
                    {
                        stop.0 = Some(event);
                        continue 'outer;
                    }
                }
                stops.push((Some(event), None))
            }
            ris_client::journey_details::EventType::Departure => {
                for stop in stops.iter_mut() {
                    if stop
                        .0
                        .as_ref()
                        .map(|arrival| {
                            stop.1.is_none()
                                && arrival.station.eva_number == event.station.eva_number
                                && arrival.time_schedule <= event.time_schedule
                        })
                        .unwrap_or(false)
                    {
                        stop.1 = Some(event);
                        continue 'outer;
                    }
                }
                stops.push((None, Some(event)))
            }
        }
    }

    let stops = stops
        .into_iter()
        .map(|stop| {
            let arrival = stop.0;
            let departure = stop.1;

            let departure_arrival = departure
                .clone()
                .unwrap_or_else(|| arrival.clone().unwrap());

            let mut messages: HashSet<JourneyDetailsMessage> = HashSet::new();

            if let Some(arrival) = arrival.clone() {
                for message in arrival.messages {
                    messages.insert(message);
                }
            }

            if let Some(departure) = departure.clone() {
                for message in departure.messages {
                    messages.insert(message);
                }
            }

            let custom_operator_name =
                match departure_arrival.administration.administration_id.as_str() {
                    "80" => "DB Fernverkehr AG",
                    "82" => "CFL",
                    "87" => "SNCF",
                    "88" => "SNCB",
                    _ => &departure_arrival.administration.operator_name,
                };

            RisJourneyStop {
                stop_id: departure_arrival.station.eva_number,
                stop_name: departure_arrival.station.name,
                arrival: arrival.map(|arrival| JourneyStopTime {
                    scheduled: arrival.time_schedule,
                    realtime: arrival.time,
                    time_type: arrival.time_type,
                }),
                departure: departure.map(|departure| JourneyStopTime {
                    scheduled: departure.time_schedule,
                    realtime: departure.time,
                    time_type: departure.time_type,
                }),
                transport: departure_arrival.transport,
                messages: messages.into_iter().collect(),
                disruptions: departure_arrival
                    .disruptions
                    .into_iter()
                    .map(|disruption| JourneyStopDisruption {
                        id: disruption.disruption_id,
                        communication_id: disruption.disruption_communication_id,
                        text: disruption.descriptions.de.text,
                        text_short: disruption.descriptions.de.text_short,
                        priority: disruption.display_priority,
                    })
                    .collect(),
                on_demand: departure_arrival.on_demand,
                cancelled: departure_arrival.canceled,
                additional: departure_arrival.additional,
                scheduled_platform: departure_arrival.platform_schedule,
                real_platform: departure_arrival.platform,
                administration: JourneyStopAdministration {
                    id: departure_arrival.administration.administration_id,
                    name: custom_operator_name.to_string(),
                    operator_code: departure_arrival.administration.operator_code,
                    ris_name: departure_arrival.administration.operator_name,
                },
            }
        })
        .collect();

    let response = RisJourneyDetails {
        id: response.journey_id,
        destination_id: response.destination_schedule.eva_number,
        destination_name: response.destination_schedule.name,
        origin_id: response.origin_schedule.eva_number,
        origin_name: response.origin_schedule.name,
        journey_type: response.r#type,
        cancelled: response.journey_canceled,
        stops,
    };

    {
        let response = response.clone();
        tokio::spawn(async move { response.insert_to_cache(state.cache.as_ref()).await });
    }

    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyDetails {
    pub id: String,
    pub journey_type: String,
    pub origin_name: String,
    pub origin_id: String,
    pub destination_name: String,
    pub destination_id: String,
    pub cancelled: bool,
    pub stops: Vec<RisJourneyStop>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RisJourneyStop {
    pub stop_id: String,
    pub stop_name: String,
    #[schema(nullable)]
    pub arrival: Option<JourneyStopTime>,
    #[schema(nullable)]
    pub departure: Option<JourneyStopTime>,
    pub messages: Vec<JourneyDetailsMessage>,
    pub disruptions: Vec<JourneyStopDisruption>,
    pub transport: Transport,
    pub on_demand: bool,
    pub cancelled: bool,
    pub additional: bool,
    #[schema(nullable)]
    pub scheduled_platform: Option<String>,
    #[schema(nullable)]
    pub real_platform: Option<String>,
    pub administration: JourneyStopAdministration,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JourneyStopTime {
    pub scheduled: DateTime<FixedOffset>,
    #[schema(nullable)]
    pub realtime: Option<DateTime<FixedOffset>>,
    pub time_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JourneyStopAdministration {
    pub id: String,
    pub name: String,
    pub operator_code: String,
    pub ris_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JourneyStopDisruption {
    pub id: String,
    pub communication_id: Option<String>,
    pub priority: i32,
    pub text: String,
    #[schema(nullable)]
    pub text_short: Option<String>,
}
