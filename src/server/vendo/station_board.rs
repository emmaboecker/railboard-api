use std::collections::BTreeMap;

use railboard_api::client::vendo::{
    station_board::{
        StationBoard, StationBoardArrivalsElement, StationBoardDeparturesElement, StationBoardError,
    },
    VendoClient,
};
use rocket::{get, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::server::{
    error::{ErrorDomain, RailboardApiError},
    response::ApiResponse,
    types::Time,
};

#[get("/station_board/<id>")]
pub async fn station_board(id: &str) -> Json<ApiResponse<Vec<StationBoardTrain>>> {
    let vendo_client = VendoClient::default();

    let arrivals = match vendo_client.station_board_arrivals(id, None, None).await {
        Ok(response) => response,
        Err(err) => {
            let error = match err {
                StationBoardError::FailedRequest(err) => RailboardApiError {
                    domain: ErrorDomain::Request,
                    message: format!("Failed to get arrivals from Vendo: {}", err),
                    error: None,
                },
                StationBoardError::VendoError(err) => RailboardApiError {
                    domain: ErrorDomain::Vendo,
                    message: format!("Failed to get arrivals from Vendo: {}", err),
                    error: Some(serde_json::to_value(err).unwrap()),
                },
            };
            return Json(ApiResponse::Error(error));
        }
    };

    let departures = match vendo_client.station_board_departures(id, None, None).await {
        Ok(response) => response,
        Err(err) => {
            let error = match err {
                StationBoardError::FailedRequest(err) => RailboardApiError {
                    domain: ErrorDomain::Request,
                    message: format!("Failed to get departures from Vendo: {}", err),
                    error: None,
                },
                StationBoardError::VendoError(err) => RailboardApiError {
                    domain: ErrorDomain::Vendo,
                    message: format!("Failed to get departures from Vendo: {}", err),
                    error: Some(serde_json::to_value(err).unwrap()),
                },
            };
            return Json(ApiResponse::Error(error));
        }
    };

    let mut trains: BTreeMap<
        String,
        (
            Option<StationBoardArrivalsElement>,
            Option<StationBoardDeparturesElement>,
        ),
    > = BTreeMap::new();

    for train in arrivals.arrivals {
        let id = train.id.clone();
        trains.entry(id).or_insert_with(|| (None, None)).0 = Some(train);
    }

    for train in departures.departures {
        let id = train.id.clone();
        trains.entry(id).or_insert_with(|| (None, None)).1 = Some(train);
    }

    let trains: Vec<StationBoardTrain> = trains
        .into_iter()
        .map(|(id, (arrival, departure))| {
            let arrival_data = arrival.clone().map(|arrival| StationBoardArrival {
                origin: arrival.origin_name,
                time: Time {
                    scheduled: arrival.arrival_date,
                    realtime: arrival.realtime_arrival_date,
                },
            });
            let departure_data = departure.clone().map(|departure| StationBoardDeparture {
                destination: departure.destination_name,
                time: Time {
                    scheduled: departure.departure_date,
                    realtime: departure.realtime_departure_date,
                },
            });

            if let Some(departure) = departure {
                StationBoardTrain {
                    journey_id: id,
                    arrival: arrival_data,
                    departure: departure_data,
                    product_type: departure.product_type,
                    short_name: departure.short_name,
                    name: departure.name,
                    scheduled_platform: departure.platform,
                    realtime_platform: departure.realtime_platform,
                    notes: departure.notes.into_iter().map(|note| note.text).collect(),
                }
            } else if let Some(arrival) = arrival {
                StationBoardTrain {
                    journey_id: id,
                    arrival: arrival_data,
                    departure: departure_data,

                    product_type: arrival.product_type,
                    short_name: arrival.short_name,
                    name: arrival.name,
                    scheduled_platform: arrival.platform,
                    realtime_platform: arrival.realtime_platform,
                    notes: arrival.notes.into_iter().map(|note| note.text).collect(),
                }
            } else {
                panic!("Arrival and departure are both None"); // This should never happen (it is just simply not possible)
            }
        })
        .collect();

    Json(ApiResponse::Success(trains))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardTrain {
    pub journey_id: String,
    pub arrival: Option<StationBoardArrival>,
    pub departure: Option<StationBoardDeparture>,
    pub product_type: String,
    pub short_name: String,
    pub name: String,
    pub scheduled_platform: Option<String>,
    pub realtime_platform: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardArrival {
    origin: String,
    time: Time,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardDeparture {
    destination: String,
    time: Time,
}
