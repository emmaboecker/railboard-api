use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, FixedOffset, TimeZone};
use chrono_tz::Europe::Berlin;
use serde::Deserialize;

use zugportal_client::station_board::ZugportalStationBoard;

use crate::{cache::{CachableObject, Cache}, error::RailboardResult, SharedState};

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
("timeStart" = Option < String >, Query, description = "The Start Time of the Time Range you are requesting"),
("timeEnd" = Option < String >, Query, description = "The End Time of the Time Range you are requesting")
),
tag = "Zugportal",
responses(
(status = 200, description = "The requested Station Board", body = RisStationBoard),
(status = 400, description = "The Error returned by the Zugportal API (Ris), will be the Ris Domain with UnderlyingApiError Variant 5", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
)
)]
#[deprecated]
pub async fn zugportal_station_board(
    Path(eva): Path<String>,
    Query(query): Query<StationBoardQuery>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<ZugportalStationBoard>> {
    let time_start = query
        .time_start
        .map(|time_start| Berlin.from_utc_datetime(&time_start.naive_utc()));

    let time_end = query
        .time_end
        .map(|time_end| Berlin.from_utc_datetime(&time_end.naive_utc()));

    if let (Some(time_start), Some(time_end)) = (time_start, time_end) {
        if let Some(cached) = state.cache
            .get_from_id(&format!(
                "zugportal.station-board.{}.{}.{}",
                eva,
                time_start.naive_utc().format("%Y-%m-%dT%H:%M"),
                time_end.naive_utc().format("%Y-%m-%dT%H:%M")
            ))
            .await
        {
            return Ok(Json(cached));
        }
    }

    let station_board = state.zugportal_client
        .station_board(&eva, time_start, time_end)
        .await?;

    {
        let station_board = station_board.clone();
        tokio::spawn(async move {
            let _ = station_board.insert_to_cache(&state.cache, None).await;
        });
    }

    Ok(Json(station_board))
}


