use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use serde::Deserialize;
use utoipa::IntoParams;

use vendo_client::station_board::VendoStationBoard;

use crate::{
    cache::{CachableObject, Cache},
    error::{ErrorDomain, RailboardApiError, RailboardResult},
    SharedState,
};

#[derive(Deserialize, IntoParams)]
pub struct StationBoardQuery {
    /// The date (Unix Timestamp) to request the station board for. If not provided, the current date is used.
    pub date: Option<i64>,
}

#[utoipa::path(
get,
path = "/vendo/v1/station_board/{id}",
params(
("id" = String, Path, description = "The eva number or location id of the Station you are requesting"),
StationBoardQuery
),
tag = "Vendo",
responses(
(status = 200, description = "The requested Station Board", body = VendoStationBoard),
(status = 400, description = "The Error returned by Vendo", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails", body = RailboardApiError)
)
)]
pub async fn station_board(
    Path(id): Path<String>,
    Query(params): Query<StationBoardQuery>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<VendoStationBoard>> {
    let date = if let Some(date) = params.date {
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
            "vendo.station-board.{}.{}.{}",
            id,
            date.format("%Y-%m-%d"),
            date.format("%H:%M")
        ))
        .await
    {
        return Ok(Json(cached));
    }

    let station_board = state.vendo_client.station_board(&id, date).await?;

    {
        let station_board = station_board.clone();
        tokio::spawn(async move { station_board.insert_to_cache(&state.cache, None).await });
    }

    Ok(Json(station_board))
}
