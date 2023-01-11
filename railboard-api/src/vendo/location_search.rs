use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use vendo_client::location_search::LocationSearchError;

use crate::{
    cache::CachableObject,
    error::{ErrorDomain, RailboardApiError, RailboardResult},
};

use super::VendoState;

pub async fn location_search(
    Path(query): Path<String>,
    State(state): State<Arc<VendoState>>,
) -> RailboardResult<Json<Vec<vendo_client::location_search::LocationSearchResult>>> {
    #[cfg(feature = "cache")]
    if let Some(cached) = LocationSearchCache::get_from_id::<LocationSearchCache>(
        &format!("location-search.{}", query),
        &state.cache,
    )
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

    #[cfg(feature = "cache")]
    tokio::spawn(async move { location_search.insert_to_cache(&state.cache).await });

    Ok(Json(result))
}

impl From<LocationSearchError> for RailboardApiError {
    fn from(value: LocationSearchError) -> Self {
        match value {
            LocationSearchError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: None,
            },
            LocationSearchError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: Some(serde_json::to_value(err).unwrap()),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationSearchCache {
    pub query: String,
    pub results: Vec<vendo_client::location_search::LocationSearchResult>,
}
