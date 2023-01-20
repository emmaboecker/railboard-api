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
