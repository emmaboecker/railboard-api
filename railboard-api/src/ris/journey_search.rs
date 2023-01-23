use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use ris_client::journey_search::RisJourneySearchElement;
use serde::Deserialize;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
};

use super::RisState;

#[derive(Deserialize)]
pub struct JounreySearchPath {
    pub number: String,
    pub category: String,
}

#[utoipa::path(
    get,
    path = "/ris/v1/journey_search/{category}/{number}",
    params(
        ("number" = String, Path, description = "The number of this Train (e.g. for ICE 2929 it would be 2929 and for RE 1 it could be 4570)"), 
        ("category" = String, Path, description = "The category of this Train (e.g. ICE, IC, RE, VIA, ...)")
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Journey Details", body = [RisJourneySearchElement]),
        (status = 400, description = "The Error returned by Ris, will be the Ris Domain with UnderlyingApiError Variant 3 or 4", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn journey_search(
    Path(path): Path<JounreySearchPath>,
    State(state): State<Arc<RisState>>,
) -> RailboardResult<Json<Vec<RisJourneySearchElement>>> {
    let category = path.category;
    let number = path.number;

    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.journey-search.{}.{}", &category, &number))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.ris_client.journey_search(&category, &number).await?;

    {
        let response = response.clone();
        tokio::spawn(async move {
            let cache = state.cache.clone();
            let category = category.clone();
            let number = number.clone();
            (category, number, response)
                .insert_to_cache(cache.clone().as_ref())
                .await
        });
    }

    Ok(Json(response.journeys))
}
