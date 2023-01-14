use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
};

use super::VendoState;

pub async fn location_search(
    Path(query): Path<String>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<Vec<vendo_client::location_search::LocationSearchResult>>> {
    if let Some(cached) = state
        .cache
        .get_from_id::<LocationSearchCache>(&format!("location-search.{}", query))
        .await
    {
        return Ok(Json(cached.results));
    }

    let result = state
        .vendo_client
        .location_search(query.clone(), None)
        .await?;

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
    pub results: Vec<vendo_client::location_search::LocationSearchResult>,
}
