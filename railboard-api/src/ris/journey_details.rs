use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use ris_client::journey_details::RisJourneyDetails;

use crate::{
    cache::{CachableObject, Cache},
    error::RailboardResult,
    SharedState,
};

#[utoipa::path(
    get,
    path = "/ris/v1/journey_details/{id}",
    params(
("id" = String, Path, description = "The id of this journey (can be optained e.G. through the journey search endpoint)")
    ),
    tag = "Ris",
    responses(
(status = 200, description = "The requested Journey Details", body = RisJourneyDetails),
(status = 400, description = "The Error returned by Ris, will be the Ris Domain with UnderlyingApiError Variant 3 or 4", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    ),
)]
#[allow(deprecated)]
#[deprecated(note = "the endpoint is not being maintained anymore, see ris-client")]
pub async fn journey_details(
    Path(id): Path<String>,
    state: State<Arc<SharedState>>,
) -> RailboardResult<Json<RisJourneyDetails>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.journey-details.{}", &id))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.ris_client.journey_details(&id).await?;

    {
        let response = response.clone();
        tokio::spawn(async move { response.insert_to_cache(&state.cache, None).await });
    }

    Ok(Json(response))
}
