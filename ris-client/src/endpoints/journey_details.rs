use std::collections::HashSet;

use serde::Deserialize;

pub use transformed::*;

use crate::{RisClient, RisError, RisOrRequestError, RisUnauthorizedError};
use crate::journey_details::response::{EventType, JourneyDetailsEvent, JourneyDetailsResponse};

mod response;
mod transformed;

impl RisClient {
    pub async fn journey_details(
        &self,
        id: &str,
    ) -> Result<RisJourneyDetails, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/db/apis/ris-journeys/v1/eventbased/{}",
            self.base_url, id,
        );

        let response: JourneyDetailsResponse = self
            .client
            .get(&url)
            .header("db-api-key", self.db_api_key.clone())
            .header("db-client-id", self.db_client_id.clone())
            .send()
            .await?
            .json()
            .await?;

        let mut stops: Vec<(Option<JourneyDetailsEvent>, Option<JourneyDetailsEvent>)> = Vec::new();

        'outer: for event in response.events {
            match event.r#type {
                EventType::Arrival => {
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
                EventType::Departure => {
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

                let mut messages: HashSet<RisJourneyDetailsMessage> = HashSet::new();

                if let Some(arrival) = arrival.clone() {
                    for message in arrival.messages {
                        messages.insert(message.into());
                    }
                }

                if let Some(departure) = departure.clone() {
                    for message in departure.messages {
                        messages.insert(message.into());
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
                    arrival: arrival.map(|arrival| RisJourneyStopEvent {
                        cancelled: arrival.canceled,
                        additional: arrival.additional,
                        on_demand: arrival.on_demand,
                        scheduled: arrival.time_schedule,
                        realtime: arrival.time,
                        time_type: arrival.time_type,
                    }),
                    departure: departure.map(|departure| RisJourneyStopEvent {
                        cancelled: departure.canceled,
                        additional: departure.additional,
                        on_demand: departure.on_demand,
                        scheduled: departure.time_schedule,
                        realtime: departure.time,
                        time_type: departure.time_type,
                    }),
                    transport: departure_arrival.transport.into(),
                    messages: messages.into_iter().collect(),
                    disruptions: departure_arrival
                        .disruptions
                        .into_iter()
                        .map(|disruption| RisJourneyStopDisruption {
                            id: disruption.disruption_id,
                            communication_id: disruption.disruption_communication_id,
                            text: disruption.descriptions.de.text,
                            text_short: disruption.descriptions.de.text_short,
                            priority: disruption.display_priority,
                        })
                        .collect(),
                    scheduled_platform: departure_arrival.platform_schedule,
                    real_platform: departure_arrival.platform,
                    administration: RisJourneyStopAdministration {
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

        Ok(response)

        // match response {
        //     RisJourneyDetailsOrErrorResponse::Response(response) => Ok(*response),
        //     RisJourneyDetailsOrErrorResponse::Error(error) => {
        //         Err(RisOrRequestError::RisError(error))
        //     }
        //     RisJourneyDetailsOrErrorResponse::UnauthorizedError(error) => {
        //         Err(RisOrRequestError::RisUnauthorizedError(error))
        //     }
        // }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RisJourneyDetailsOrErrorResponse {
    Response(Box<JourneyDetailsResponse>),
    Error(RisError),
    UnauthorizedError(RisUnauthorizedError),
}
