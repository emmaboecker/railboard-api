use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use iris_client::station_board::{message::Message, RouteStop};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    error::RailboardResult, iris::station_board::iris_station_board,
    ris::station_board::ris_station_board,
};

use super::CustomEndpointState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardQuery {
    pub time_start: Option<DateTime<FixedOffset>>,
    pub time_end: Option<DateTime<FixedOffset>>,
}

#[utoipa::path(
    get,
    path = "/v1/station_board/{eva}",
    params(
        ("eva" = String, Path, description = "The Eva Number of the Station you are requesting"),
        ("timeStart" = Option<DateTime<FixedOffset>>, Query, description = "The Start Time of the Time Range you are requesting"),
        ("timeEnd" = Option<DateTime<FixedOffset>>, Query, description = "The End Time of the Time Range you are requesting")
    ),
    tag = "Custom",
    responses(
        (status = 200, description = "The requested Station Board", body = StationBoard),
        (status = 400, description = "The Error returned by the Ris or Iris, will be Variant 2 or Variant 5", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn station_board(
    Path(eva): Path<String>,
    Query(query): Query<StationBoardQuery>,
    State(state): State<Arc<CustomEndpointState>>,
) -> RailboardResult<Json<StationBoard>> {
    let time_start = if let Some(time_start) = query.time_start {
        Berlin.from_utc_datetime(&time_start.naive_utc())
    } else {
        Berlin.from_utc_datetime(&Utc::now().naive_utc())
    };

    let time_end = if let Some(time_end) = query.time_end {
        Berlin.from_utc_datetime(&time_end.naive_utc())
    } else {
        Berlin.from_utc_datetime(&(Utc::now().naive_utc() + chrono::Duration::minutes(30)))
    };

    let (ris_station_board, iris_station_board) = tokio::join!(
        ris_station_board(
            &eva,
            Some(time_start),
            Some(time_end),
            state.ris_client.clone(),
            state.cache.clone()
        ),
        iris_station_board(
            &eva,
            time_end,
            time_start,
            state.iris_client.clone(),
            state.cache.clone()
        )
    );

    let ris_station_board = ris_station_board?;
    let iris_station_board = iris_station_board?;

    let items = ris_station_board.items;

    let items: Vec<StationBoardItem> = items
        .into_iter()
        .map(|item| {
            let iris_item = iris_station_board.stops.iter().find(|iris_item| {
                iris_item.train_number == item.train_number.to_string()
                    && (iris_item
                        .arrival
                        .clone()
                        .map(|arrival| arrival.planned_time)
                        == item.arrival.clone().map(|arrival| arrival.time_scheduled)
                        || iris_item
                            .departure
                            .clone()
                            .map(|departure| departure.planned_time)
                            == item
                                .departure
                                .clone()
                                .map(|departure| departure.time_scheduled))
            });

            let iris_item = iris_item.cloned();

            StationBoardItem {
                journey_id: item.journey_id,

                station_eva: item.station_eva,
                station_name: item.station_name,

                category: item.category,
                train_type: item.train_type,
                train_number: item.train_number,
                line_indicator: item.line_indicator,

                cancelled: item.cancelled,

                arrival: item.arrival.map(|arrival| DepartureArrival {
                    time_scheduled: arrival.time_scheduled,
                    time_realtime: arrival.time_realtime,
                    delay: arrival.delay,
                    time_type: arrival.time_type,
                    wings: iris_item
                        .clone()
                        .and_then(|iris| iris.arrival.map(|arrival| arrival.wings))
                        .unwrap_or_default(),
                }),
                departure: item.departure.map(|departure| DepartureArrival {
                    time_scheduled: departure.time_scheduled,
                    time_realtime: departure.time_realtime,
                    delay: departure.delay,
                    time_type: departure.time_type,
                    wings: iris_item
                        .clone()
                        .and_then(|iris| iris.departure.map(|departure| departure.wings))
                        .unwrap_or_default(),
                }),

                platform_scheduled: item.platform_scheduled,
                platform_realtime: item.platform_realtime,

                origin_eva: item.origin_eva,
                origin_name: item.origin_name,
                destination_eva: item.destination_eva,
                destination_name: item.destination_name,

                administation: StationBoardItemAdministration {
                    id: item.administation.id,
                    operator_code: item.administation.operator_code,
                    operator_name: item.administation.operator_name,
                    ris_operator_name: item.administation.ris_operator_name,
                },

                additional_info: iris_item.map(|iris| IrisInformation {
                    iris_id: iris.id,
                    replaces: iris
                        .replaces
                        .map(|replaces| format!("{} {}", replaces.category, replaces.number)),
                    route: iris.route,
                    messages: iris.messages,
                }),
            }
        })
        .collect();

    let station_board = StationBoard {
        eva: ris_station_board.eva,
        name: ris_station_board.name,
        time_start: ris_station_board.time_start,
        time_end: ris_station_board.time_end,
        items,
    };

    Ok(Json(station_board))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoard {
    pub eva: String,
    pub name: String,
    pub time_start: DateTime<FixedOffset>,
    pub time_end: DateTime<FixedOffset>,
    pub items: Vec<StationBoardItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItem {
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

    pub administation: StationBoardItemAdministration,

    #[schema(nullable)]
    pub additional_info: Option<IrisInformation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IrisInformation {
    pub iris_id: String,
    #[schema(nullable)]
    pub replaces: Option<String>,
    pub route: Vec<RouteStop>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationBoardItemAdministration {
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

    pub wings: Vec<String>,
}
