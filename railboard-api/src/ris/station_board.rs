use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, FixedOffset, TimeZone};
use chrono_tz::Europe::Berlin;
use ris_client::{station_board::StationBoardItem, RisClient};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    cache::{self, CachableObject, Cache},
    error::RailboardResult,
    name_from_administation_code,
};

use super::RisState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardQuery {
    pub time_start: Option<DateTime<FixedOffset>>,
    pub time_end: Option<DateTime<FixedOffset>>,
}

#[utoipa::path(
    get,
    path = "/ris/v1/station_board/{eva}",
    params(
        ("eva" = String, Path, description = "The Eva Number of the Station you are requesting"),
        ("timeStart" = Option<String>, Query, description = "The Start Time of the Time Range you are requesting"),
        ("timeEnd" = Option<String>, Query, description = "The End Time of the Time Range you are requesting")
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Station Board", body = RisStationBoard),
        (status = 400, description = "The Error returned by the Zugportal API (Ris), will be the Ris Domain with UnderlyingApiError Variant 5", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn station_board(
    Path(eva): Path<String>,
    Query(query): Query<StationBoardQuery>,
    State(state): State<Arc<RisState>>,
) -> RailboardResult<Json<RisStationBoard>> {
    let time_start = query
        .time_start
        .map(|time_start| Berlin.from_utc_datetime(&time_start.naive_utc()));

    let time_end = query
        .time_end
        .map(|time_end| Berlin.from_utc_datetime(&time_end.naive_utc()));

    let result = ris_station_board(
        &eva,
        time_start,
        time_end,
        state.ris_client.clone(),
        state.cache.clone(),
    )
    .await?;

    Ok(Json(result))
}

pub async fn ris_station_board(
    eva: &str,
    time_start: Option<DateTime<chrono_tz::Tz>>,
    time_end: Option<DateTime<chrono_tz::Tz>>,
    ris_client: Arc<RisClient>,
    cache: Arc<cache::RedisCache>,
) -> RailboardResult<RisStationBoard> {
    if let (Some(time_start), Some(time_end)) = (time_start, time_end) {
        if let Some(cached) = cache
            .get_from_id(&format!(
                "ris.station-board.{}.{}.{}",
                eva,
                time_start.naive_utc().format("%Y-%m-%dT%H:%M"),
                time_end.naive_utc().format("%Y-%m-%dT%H:%M")
            ))
            .await
        {
            return Ok(cached);
        }
    }

    let (arrivals, departures) = tokio::join!(
        ris_client.station_board_arrivals(eva, time_start, time_end),
        ris_client.station_board_departures(eva, time_start, time_end)
    );

    let arrivals = arrivals?;
    let departures = departures?;

    let mut trains: BTreeMap<String, (Option<StationBoardItem>, Option<StationBoardItem>)> =
        BTreeMap::new();

    for train in arrivals.items {
        let id = train.train.journey_id.to_owned();
        trains.entry(id).or_insert_with(|| (None, None)).0 = Some(train);
    }

    for train in departures.items {
        let id = train.train.journey_id.to_owned();
        trains.entry(id).or_insert_with(|| (None, None)).1 = Some(train);
    }

    let station_board = RisStationBoard {
        eva: departures.eva_no,
        name: departures.station_name,
        time_start: departures.time_start,
        time_end: departures.time_end,
        items: trains
            .into_iter()
            .map(|(id, (arrival, departure))| {
                let departure_arrival = departure
                    .clone()
                    .unwrap_or_else(|| arrival.clone().unwrap());

                let scheduled_platform = if departure_arrival.platform.is_empty() {
                    None
                } else {
                    Some(departure_arrival.platform)
                };
                let realtime_platform = if departure_arrival.platform_predicted.is_empty() {
                    None
                } else {
                    Some(departure_arrival.platform_predicted)
                };

                RisStationBoardItem {
                    journey_id: id,
                    station_eva: departure_arrival.station.eva_no.clone(),
                    station_name: departure_arrival.station.name.clone(),
                    cancelled: departure_arrival.canceled || departure_arrival.station.canceled,
                    category: departure_arrival.train.category,
                    train_type: departure_arrival.train.r#type,
                    train_number: departure_arrival.train.no,
                    line_indicator: departure_arrival.train.line_name,
                    departure: departure.as_ref().map(|departure| DepartureArrival {
                        delay: departure.diff,
                        time_realtime: departure.time_predicted,
                        time_scheduled: departure.time,
                        time_type: departure.time_type.clone(),
                    }),
                    arrival: arrival.as_ref().map(|arrival| DepartureArrival {
                        delay: arrival.diff,
                        time_realtime: arrival.time_predicted,
                        time_scheduled: arrival.time,
                        time_type: arrival.time_type.clone(),
                    }),
                    destination_eva: departure
                        .as_ref()
                        .and_then(|departure| {
                            departure
                                .destination
                                .as_ref()
                                .map(|destination| destination.eva_no.clone())
                        })
                        .unwrap_or(departure_arrival.station.eva_no.clone()),
                    destination_name: departure
                        .and_then(|departure| {
                            departure.destination.map(|destination| destination.name)
                        })
                        .unwrap_or(departure_arrival.station.name.clone()),
                    origin_eva: arrival
                        .as_ref()
                        .and_then(|arrival| {
                            arrival.origin.as_ref().map(|origin| origin.eva_no.clone())
                        })
                        .unwrap_or(departure_arrival.station.eva_no),
                    origin_name: arrival
                        .as_ref()
                        .and_then(|arrival| {
                            arrival.origin.as_ref().map(|origin| origin.name.clone())
                        })
                        .unwrap_or(departure_arrival.station.name),
                    platform_scheduled: scheduled_platform,
                    platform_realtime: realtime_platform,
                    administation: RisStationBoardItemAdministration {
                        id: departure_arrival.administration.id,
                        operator_code: departure_arrival.administration.operator_code,
                        operator_name: String::from(
                            name_from_administation_code(
                                &departure_arrival.administration.operator_name,
                            )
                            .unwrap_or(&departure_arrival.administration.operator_name),
                        ),
                        ris_operator_name: departure_arrival.administration.operator_name,
                    },
                }
            })
            .collect(),
    };

    {
        let station_board = station_board.clone();
        tokio::spawn(async move {
            let _ = station_board.insert_to_cache(cache.as_ref()).await;
        });
    }

    Ok(station_board)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoard {
    pub eva: String,
    pub name: String,
    pub time_start: DateTime<FixedOffset>,
    pub time_end: DateTime<FixedOffset>,
    pub items: Vec<RisStationBoardItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoardItem {
    pub journey_id: String,

    pub station_eva: String,
    pub station_name: String,

    pub category: String,
    pub train_type: String,
    pub train_number: u32,
    pub line_indicator: String,

    pub cancelled: bool,

    pub arrival: Option<DepartureArrival>,
    pub departure: Option<DepartureArrival>,

    #[schema(nullable)]
    pub platform_scheduled: Option<String>,
    #[schema(nullable)]
    pub platform_realtime: Option<String>,

    pub origin_eva: String,
    pub origin_name: String,
    pub destination_eva: String,
    pub destination_name: String,

    pub administation: RisStationBoardItemAdministration,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisStationBoardItemAdministration {
    pub id: String,
    pub operator_code: String,
    pub operator_name: String,
    pub ris_operator_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepartureArrival {
    /// Since ris returns dates with seconds it also rounds up this number if the seconds are 50 for example
    pub delay: i32,
    pub time_scheduled: DateTime<FixedOffset>,
    pub time_realtime: DateTime<FixedOffset>,
    pub time_type: String,
}
