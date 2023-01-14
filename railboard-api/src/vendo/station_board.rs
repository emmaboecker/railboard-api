use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use serde::{Deserialize, Serialize};
use vendo_client::station_board::{StationBoardArrivalsElement, StationBoardDeparturesElement};

use crate::{
    cache::{CachableObject, Cache},
    error::{ErrorDomain, RailboardApiError, RailboardResult},
    types::Time,
};

use super::VendoState;

pub async fn station_board(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<StationBoard>> {
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

    if let Some(cached) = state
        .cache
        .get_from_id(&format!(
            "station-board.{}.{}.{}",
            id,
            date.format("%Y-%m-%d"),
            date.format("%H:%M")
        ))
        .await
    {
        return Ok(Json(cached));
    }

    let (arrivals, departures) = tokio::join!(
        state
            .vendo_client
            .station_board_arrivals(&id, Some(date), None),
        state
            .vendo_client
            .station_board_departures(&id, Some(date), None)
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

    let mut trains: Vec<StationBoardElement> = trains
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
                StationBoardElement {
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
                StationBoardElement {
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

    trains.sort_by(|a, b| {
        let a_dep = a.departure.clone().map(|departure| departure.time);
        let a_arr = a.arrival.clone().map(|arrival| arrival.time);
        let b_dep = b.departure.clone().map(|departure| departure.time);
        let b_arr = b.arrival.clone().map(|arrival| arrival.time); // I need to remove these clones but I don't know how
        a_dep
            .unwrap_or_else(|| a_arr.unwrap())
            .scheduled
            .cmp(&b_dep.unwrap_or_else(|| b_arr.unwrap()).scheduled)
    });

    let station_board = StationBoard {
        day: date.format("%Y-%m-%d").to_string(),
        time: date.format("%H:%M").to_string(),
        id,
        station_board: trains,
    };

    {
        let station_board = station_board.clone();
        tokio::spawn(async move { station_board.insert_to_cache(&state.cache).await });
    }

    Ok(Json(station_board))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoard {
    pub day: String,
    pub time: String,
    pub id: String,
    pub station_board: Vec<StationBoardElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardElement {
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
