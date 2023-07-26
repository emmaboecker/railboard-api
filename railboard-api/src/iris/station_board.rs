use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Timelike};
use chrono_tz::{Europe::Berlin, Tz};
use iris_client::{
    station_board::{from_iris_timetable, response::TimeTable, IrisStationBoard},
    IrisClient, IrisOrRequestError,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    cache::{self, CachableObject, Cache},
    error::RailboardResult,
};

use super::IrisState;

#[derive(Deserialize, IntoParams)]
pub struct IrisStationBoardQuery {
    /// The date to request the station board for. If not provided, the current date is used.
    pub date: Option<DateTime<FixedOffset>>,
    /// The time to request data for in the past
    pub lookbehind: Option<u32>,
    /// The time to request data for in the future
    pub lookahead: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/iris/v1/station_board/{eva}",
    params(
        ("eva" = String, Path, description = "The eva number of the Station you are requesting"),
        IrisStationBoardQuery
    ),
    tag = "Iris",
    responses(
        (status = 200, description = "The requested Station Board", body = IrisStationBoard),
        (status = 400, description = "The Error returned by Iris, will be the Iris Domain with UnderlyingApiError Variant 2, which has no Information because Iris doesn't return errors", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails", body = RailboardApiError)
    )
)]
pub async fn station_board(
    Path(eva): Path<String>,
    Query(params): Query<IrisStationBoardQuery>,
    State(state): State<Arc<IrisState>>,
) -> RailboardResult<Json<iris_client::station_board::IrisStationBoard>> {
    let lookbehind = params.lookbehind.unwrap_or(20);
    let lookahead = params.lookahead.unwrap_or(180);

    let date = if let Some(date) = params.date {
        Berlin.from_utc_datetime(&date.naive_utc())
    } else {
        Berlin.from_utc_datetime(&chrono::Utc::now().naive_utc())
    };

    let lookbehind = date - chrono::Duration::minutes(lookbehind as i64);
    let lookahead = date + chrono::Duration::minutes(lookahead as i64);

    let station_board = iris_station_board(
        &eva,
        lookahead,
        lookbehind,
        state.iris_client.clone(),
        state.cache.clone(),
    )
    .await?;

    Ok(Json(station_board))
}

pub async fn iris_station_board(
    eva: &str,
    lookahead: DateTime<Tz>,
    lookbehind: DateTime<Tz>,
    iris_client: Arc<IrisClient>,
    cache: Arc<cache::RedisCache>,
) -> RailboardResult<iris_client::station_board::IrisStationBoard> {
    let mut dates = Vec::new();

    for current_date in DateRange(lookbehind, lookahead) {
        dates.push(current_date);
    }

    let (realtime, timetables) = tokio::join!(
        get_realtime(iris_client.clone(), cache.clone(), eva),
        futures::future::join_all(dates.iter().map(|date| async {
            if let Some(cached) = cache
                .as_ref()
                .get_from_id::<iris_client::station_board::response::TimeTable>(&format!(
                    "iris.station-board.plan.{}.{}.{}",
                    eva,
                    date.format("%Y-%m-%d"),
                    date.format("%H")
                ))
                .await
            {
                return Ok(cached);
            }
            let timetable = iris_client
                .as_ref()
                .planned_station_board(
                    eva,
                    &date.format("%y%m%d").to_string(),
                    &date.format("%H").to_string(),
                )
                .await;
            match timetable {
                Ok(timetable) => {
                    let cache_timetable = (
                        timetable.clone(),
                        eva.to_string(),
                        date.format("%Y-%m-%d").to_string(),
                        date.format("%H").to_string(),
                    );
                    let cache = cache.clone();
                    tokio::spawn(async move {
                        cache_timetable.insert_to_cache(cache.as_ref(), None).await
                    });
                    Ok(timetable)
                }
                Err(err) => Err(err),
            }
        }))
    );

    let realtime = realtime?;
    let timetables = timetables
        .into_iter()
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>();

    let disruptions = realtime
        .disruptions
        .into_iter()
        .map(|message| message.into())
        .collect::<Vec<iris_client::station_board::message::Message>>();

    let mut stops = Vec::new();

    // TODO: find additional stops in realtime that are not in planned
    // realtime.stops.iter().filter(|stop| todo!());

    for timetable in timetables {
        for stop in timetable.stops {
            let realtime = realtime
                .stops
                .iter()
                .find(|realtime_stop| realtime_stop.id == stop.id);
            stops.push(from_iris_timetable(
                eva,
                &timetable.station_name,
                stop,
                realtime.map(|realtime| realtime.to_owned()),
            ));
        }
    }

    let station_board = IrisStationBoard {
        station_name: realtime.station_name,
        station_eva: String::from(eva),
        disruptions,
        stops,
    };

    Ok(station_board)
}

struct DateRange(DateTime<Tz>, DateTime<Tz>);

impl Iterator for DateRange {
    type Item = DateTime<Tz>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 || self.0.hour() == self.1.hour() {
            let next = self.0 + Duration::hours(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

async fn get_realtime(
    iris_client: Arc<IrisClient>,
    cache: Arc<cache::RedisCache>,
    id: &str,
) -> Result<TimeTable, IrisOrRequestError> {
    if let Some(cached) = &cache
        .get_from_id::<iris_client::station_board::response::TimeTable>(&format!(
            "iris.station-board.realtime.{}",
            id.to_owned()
        ))
        .await
    {
        return Ok(cached.to_owned());
    }
    let realtime = iris_client.as_ref().realtime_station_board(id).await;

    match realtime {
        Ok(realtime) => {
            let realtime = realtime;
            let cache_realtime = (realtime.clone(), id.to_owned());
            tokio::spawn(async move {
                cache_realtime
                    .insert_to_cache(cache.clone().as_ref(), None)
                    .await
            });
            Ok(realtime)
        }
        Err(err) => Err(err),
    }
}
