use std::collections::{BTreeMap, HashMap};

use axum::{
    extract::{Path, Query},
    Json,
};
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use railboard_api::client::vendo::{
    station_board::{
        StationBoardArrivalsElement, StationBoardDeparturesElement, StationBoardError,
    },
    VendoClient,
};
use serde::{Deserialize, Serialize};

use crate::server::{
    error::{ErrorDomain, RailboardApiError, RailboardResult},
    types::Time,
};

pub async fn station_board(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> RailboardResult<Json<Vec<StationBoardTrain>>> {
    let vendo_client = VendoClient::default();

    let date = params.get("date");

    let date = if let Some(date) = date {
        Some(date.parse()?)
    } else {
        None
    };

    let date = if let Some(date) = date {
        Berlin.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp_opt(date, 0).ok_or(
            RailboardApiError {
                domain: ErrorDomain::Input,
                message: "Invalid date".to_string(),
                error: None,
            },
        )?)
    } else {
        Berlin.from_utc_datetime(&chrono::Utc::now().naive_utc())
    };

    let (arrivals, departures) = tokio::join!(
        vendo_client.station_board_arrivals(&id, Some(date), None),
        vendo_client.station_board_departures(&id, Some(date), None)
    );

    let arrivals = arrivals?;
    let departures = departures?;

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

    let mut trains: Vec<StationBoardTrain> = trains
        .into_iter()
        .map(|(id, (arrival, departure))| {
            let arrival_data = arrival.as_ref().map(|arrival| StationBoardArrival {
                origin: arrival.origin_name.clone(),
                time: Time {
                    scheduled: arrival.arrival_date,
                    realtime: arrival.realtime_arrival_date,
                },
            });
            let departure_data = departure.as_ref().map(|departure| StationBoardDeparture {
                destination: departure.destination_name.clone(),
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
                panic!("Arrival and departure are both None"); // This should never happen (it is just simply not possible) // idk it's still DB 😀😀😀
            }
        })
        .collect();

    trains.sort_by(|a, b| {
        let a_dep = a.departure.clone().map(|departure| departure.time);
        let a_arr = a.arrival.clone().map(|arrival| arrival.time);
        let b_dep = b.departure.clone().map(|departure| departure.time);
        let b_arr = b.arrival.clone().map(|arrival| arrival.time);
        a_dep
            .unwrap_or_else(|| a_arr.unwrap())
            .scheduled
            .cmp(&b_dep.unwrap_or_else(|| b_arr.unwrap()).scheduled)
    });

    Ok(Json(trains))
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardArrival {
    origin: String,
    time: Time,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardDeparture {
    destination: String,
    time: Time,
}

impl From<StationBoardError> for RailboardApiError {
    fn from(value: StationBoardError) -> Self {
        match value {
            StationBoardError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: None,
            },
            StationBoardError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: Some(serde_json::to_value(err).unwrap()),
            },
        }
    }
}
