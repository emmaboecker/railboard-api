use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use vendo_client::journey_details::VendoJourneyDetails;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
    SharedState,
};

#[utoipa::path(
get,
path = "/vendo/v1/journey_details/{id}",
params(("id" = String, Path, description = "The Vendo-ID of the Journey you want to get details for")),
tag = "Vendo",
responses(
(status = 200, description = "The requested Journey Details", body = VendoJourneyDetails),
(status = 400, description = "The Error returned by Vendo", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails", body = RailboardApiError)
)
)]
pub async fn journey_details(
    Path(id): Path<String>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<VendoJourneyDetails>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("vendo.journey-details.{}", &id))
        .await
    {
        return Ok(Json(cached));
    }

    let journey_details = state.vendo_client.journey_details(&id).await?;

    {
        let cached = journey_details.clone();
        tokio::spawn(async move { cached.insert_to_cache(&state.cache, None).await });
    }

    Ok(Json(journey_details))
}
