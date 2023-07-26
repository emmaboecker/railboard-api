use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use ris_client::station_search::RisStationSearchResponse;
use serde::Deserialize;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
};

use super::RisState;

#[derive(Deserialize)]
pub struct RisStationSearchQuery {
    pub limit: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/ris/v1/station_search/{query}",
    params(
        ("query" = String, Path, description = "The Query for the station (for example: \"Lepzig Hbf\")"),
        ("limit" = Option<u32>, Query, description = "The maximum amount of results to return (default: 25)")
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Journey Details", body = RisJourneyDetails),
        (status = 400, description = "The Error returned by Ris, will be the Ris Domain with UnderlyingApiError Variant 3 or 4", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn station_search_by_name(
    Path(query): Path<String>,
    Query(query_params): Query<RisStationSearchQuery>,
    state: State<Arc<RisState>>,
) -> RailboardResult<Json<RisStationSearchResponse>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.journey-details.{}", &query))
        .await
    {
        return Ok(Json(cached));
    }

    let limit = query_params.limit;

    let response = state
        .ris_client
        .station_search_by_name(&query, limit)
        .await?;

    {
        let response = response.clone();

        let limit = limit.unwrap_or(25);

        tokio::spawn(async move {
            response
                .insert_to_cache(
                    state.cache.as_ref(),
                    Some(&format!("{}.{}", query, limit.to_string())),
                )
                .await
        });
    }

    Ok(Json(response))
}
