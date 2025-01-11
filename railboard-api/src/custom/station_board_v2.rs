use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use serde::{Deserialize, Serialize};

use iris_client::station_board::{message::Message, IrisStationBoard, RouteStop};
use utoipa::ToSchema;

use crate::{error::RailboardResult, iris::station_board::iris_station_board, SharedState};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardQuery {
    pub time_start: Option<DateTime<FixedOffset>>,
}

#[utoipa::path(
get,
path = "/v2/station_board/{eva}",  
params(
("eva" = String, Path, description = "The Eva Number of the Station you are requesting, the main difference between v1 and v2 are the datasources, v1 uses the Ris and Iris, v2 uses the Vendo and Iris"),
("timeStart" = Option < DateTime < FixedOffset >>, Query, description = "The Start Time of the Time Range you are requesting"),
),
tag = "Custom",
responses(
(status = 200, description = "The requested Station Board", body = StationBoard),
(status = 400, description = "The Error returned by the Ris or Iris, will be Variant 2 or Variant 5", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
)
)]
pub async fn station_board_v2(
    Path(eva): Path<String>,
    Query(query): Query<StationBoardQuery>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<StationBoard>> {
    let time_start = if let Some(time_start) = query.time_start {
        Berlin.from_utc_datetime(&time_start.naive_utc())
    } else {
        Berlin.from_utc_datetime(&Utc::now().naive_utc())
    };

    let time_end = Berlin.from_utc_datetime(&(time_start.naive_utc() + chrono::Duration::hours(1)));

    let (vendo_station_board, iris_station_board) = tokio::join!(
        state.vendo_client.station_board(&eva, time_start),
        iris_station_board(
            &eva,
            time_end,
            time_start,
            state.iris_client.clone(),
            &state.cache
        )
    );

    let vendo_station_board = vendo_station_board?;
    let iris_station_board = iris_station_board.unwrap_or(IrisStationBoard {
        station_name: String::new(),
        station_eva: String::new(),
        stops: vec![],
        disruptions: vec![],
    });

    let items = vendo_station_board.station_board;

    let mut items: Vec<StationBoardItem> =
        items
            .into_iter()
            .map(|item| {
                let iris_item =
                    iris_station_board.stops.iter().find(|iris_item| {
                        item.name.replace(" ", "")
                            == format!("{}{}", iris_item.train_type, iris_item.line_indicator)
                                .replace(" ", "")
                            && (iris_item
                                .arrival
                                .clone()
                                .map(|arrival| arrival.planned_time.naive_utc().date().day())
                                == item
                                    .arrival
                                    .clone()
                                    .map(|arrival| arrival.time.scheduled.naive_utc().date().day())
                                || iris_item.departure.clone().map(|departure| {
                                    departure.planned_time.naive_utc().date().day()
                                }) == item.departure.clone().map(|departure| {
                                    departure.time.scheduled.naive_utc().date().day()
                                }))
                    });

                let iris_item = iris_item.cloned();

                StationBoardItem {
                    vendo_id: Some(item.journey_id),
                    iris_id: iris_item.as_ref().map(|iris| iris.id.clone()),

                    station_eva: item.request_station.eva.clone(),
                    station_name: item.request_station.name.clone(),

                    name: item.name.clone(),
                    short_name: item.short_name.clone(),
                    category: item.product_type,
                    train_type: iris_item.clone().map(|iris| iris.train_type).unwrap_or(
                        item.name
                            .chars()
                            .take_while(|c| c.is_ascii_alphabetic())
                            .collect(),
                    ),
                    train_number: iris_item
                        .clone()
                        .map(|iris| iris.train_number.parse().unwrap_or(0)),
                    line_indicator: iris_item.clone().map(|iris| iris.line_indicator).unwrap_or(
                        item.name
                            .split_whitespace()
                            .last()
                            .unwrap_or_default()
                            .to_owned(),
                    ),

                    cancelled: item.notes.iter().any(|note| note == "Halt entfällt"),

                    arrival: item.arrival.as_ref().map(|arrival| DepartureArrival {
                        time_scheduled: arrival.time.scheduled,
                        time_realtime: arrival.time.realtime,
                        wings: iris_item
                            .clone()
                            .and_then(|iris| iris.arrival.map(|arrival| arrival.wings))
                            .unwrap_or_default(),
                    }),
                    departure: item.departure.as_ref().map(|departure| DepartureArrival {
                        time_scheduled: departure.time.scheduled,
                        time_realtime: departure.time.realtime,
                        wings: iris_item
                            .clone()
                            .and_then(|iris| iris.departure.map(|departure| departure.wings))
                            .unwrap_or_default(),
                    }),

                    platform_scheduled: item.scheduled_platform,
                    platform_realtime: item.realtime_platform,

                    origin_eva: item
                        .arrival
                        .as_ref()
                        .map(|_| None)
                        .unwrap_or(Some(item.request_station.eva.clone())),
                    origin_name: item
                        .arrival
                        .as_ref()
                        .map(|arrival| arrival.origin.clone())
                        .unwrap_or(item.request_station.name.clone()),
                    destination_eva: item
                        .departure
                        .as_ref()
                        .map(|_| None)
                        .unwrap_or(Some(item.request_station.eva)),
                    destination_name: item
                        .departure
                        .as_ref()
                        .map(|departure| departure.destination.clone())
                        .unwrap_or(item.request_station.name),

                    additional_info: iris_item.map(|iris| IrisInformation {
                        replaces: iris
                            .replaces
                            .map(|replaces| format!("{} {}", replaces.category, replaces.number)),
                        route: iris.route,
                        messages: iris.messages,
                    }),
                }
            })
            .collect();

    for stop in iris_station_board.stops.into_iter().filter(|stop| {
        stop.arrival
            .as_ref()
            .map(|arrival| {
                arrival.planned_time.naive_utc() >= time_start.naive_utc()
                    && arrival.planned_time.naive_utc() <= time_end.naive_utc()
            })
            .unwrap_or(false)
            || stop
                .departure
                .as_ref()
                .map(|departure| {
                    departure.planned_time.naive_utc() >= time_start.naive_utc()
                        && departure.planned_time.naive_utc() <= time_end.naive_utc()
                })
                .unwrap_or(false)
    }) {
        if !items
            .iter()
            .any(|item| item.iris_id == Some(stop.id.clone()))
        {
            items.push(StationBoardItem {
                vendo_id: None,
                iris_id: Some(stop.id),
                station_eva: stop.station_eva,
                station_name: stop.station_name,
                name: format!("{} {}", stop.train_type, stop.line_indicator),
                short_name: stop.train_type.clone(),
                category: stop.train_type.clone(),
                train_type: stop.train_type,
                train_number: stop.train_number.parse().map(Some).unwrap_or(None),
                line_indicator: stop.line_indicator,
                cancelled: stop.cancelled,
                arrival: stop.arrival.map(|arrival| DepartureArrival {
                    time_scheduled: arrival.planned_time,
                    time_realtime: arrival.real_time,
                    wings: arrival.wings,
                }),
                departure: stop.departure.map(|departure| DepartureArrival {
                    time_scheduled: departure.planned_time,
                    time_realtime: departure.real_time,
                    wings: departure.wings,
                }),
                platform_scheduled: stop.planned_platform,
                platform_realtime: stop.real_platform,
                origin_eva: None,
                origin_name: stop.route.first().unwrap().name.clone(),
                destination_eva: None,
                destination_name: stop.route.last().unwrap().name.clone(),
                additional_info: Some(IrisInformation {
                    replaces: stop
                        .replaces
                        .map(|replaces| format!("{} {}", replaces.category, replaces.number)),
                    route: stop.route,
                    messages: stop.messages,
                }),
            });
        }
    }

    items.sort_by(|a, b| {
        a.arrival
            .as_ref()
            .unwrap_or_else(|| a.departure.as_ref().unwrap())
            .time_scheduled
            .cmp(
                &b.arrival
                    .as_ref()
                    .unwrap_or_else(|| b.departure.as_ref().unwrap())
                    .time_scheduled,
            )
    });

    let station_board = StationBoard {
        eva,
        time_start: time_start.fixed_offset(),
        time_end: time_end.fixed_offset(),
        items,
    };

    Ok(Json(station_board))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoard {
    pub eva: String,
    pub time_start: DateTime<FixedOffset>,
    pub time_end: DateTime<FixedOffset>,
    pub items: Vec<StationBoardItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItem {
    #[schema(nullable)]
    pub vendo_id: Option<String>,
    #[schema(nullable)]
    pub iris_id: Option<String>,

    pub station_eva: String,
    pub station_name: String,

    pub name: String,
    pub short_name: String,
    pub category: String,
    pub train_type: String,
    #[schema(nullable)]
    pub train_number: Option<u32>,
    pub line_indicator: String,

    pub cancelled: bool,

    #[schema(nullable)]
    pub arrival: Option<DepartureArrival>,
    #[schema(nullable)]
    pub departure: Option<DepartureArrival>,

    #[schema(nullable)]
    pub platform_scheduled: Option<String>,
    #[schema(nullable)]
    pub platform_realtime: Option<String>,

    #[schema(nullable)]
    pub origin_eva: Option<String>,
    pub origin_name: String,
    #[schema(nullable)]
    pub destination_eva: Option<String>,
    pub destination_name: String,

    #[schema(nullable)]
    pub additional_info: Option<IrisInformation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IrisInformation {
    #[schema(nullable)]
    pub replaces: Option<String>,
    pub route: Vec<RouteStop>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepartureArrival {
    pub time_scheduled: DateTime<FixedOffset>,
    pub time_realtime: Option<DateTime<FixedOffset>>,

    pub wings: Vec<String>,
}
