use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use vendo_client::location_search::LocationSearchResult;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
};

use super::VendoState;

#[utoipa::path(
    get,
    path = "/vendo/v1/location_search/{query}",
    params(("query" = String, Path, description = "The query you want to search for")),
    tag = "Vendo",
    responses(
        (status = 200, description = "The requested Location Search Results", body = [LocationSearchResult]),
        (status = 400, description = "The Error returned by Vendo, will be the Vendo Domain with UnderlyingApiError Variant 1", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails", body = RailboardApiError)
    )
)]
pub async fn location_search(
    Path(query): Path<String>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<Vec<LocationSearchResult>>> {
    if let Some(cached) = state
        .cache
        .get_from_id::<LocationSearchCache>(&format!("vendo.location-search.{}", query))
        .await
    {
        return Ok(Json(cached.results));
    }

    let result: Vec<LocationSearchResult> = state
        .vendo_client
        .location_search(query.clone(), None)
        .await?
        .into_iter()
        .collect();

    let location_search = LocationSearchCache {
        query,
        results: result.clone(),
    };

    tokio::spawn(async move { location_search.insert_to_cache(&state.cache).await });

    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationSearchCache {
    pub query: String,
    pub results: Vec<LocationSearchResult>,
}
