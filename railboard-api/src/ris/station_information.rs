use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use ris_client::station_information::RisStationInformation;

use crate::{cache::{CachableObject, Cache}, error::{RailboardApiError, RailboardResult}, SharedState};

#[utoipa::path(
get,
path = "/ris/v1/station/{eva}",
params(
("eva" = String, Path, description = "The Eva Number of the Station you are requesting"),
),
tag = "Ris",
responses(
(status = 200, description = "The requested Station Information", body = RisStationInformation),
(status = 400, description = "The Error returned by the Ris, will be the Ris Domain with UnderlyingApiError Variant 3, 4 or none if there was no Station found", body = RailboardApiError),
(status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
)
)]
pub async fn station_information(
    Path(eva): Path<String>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<RisStationInformation>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.station-information.{}", &eva))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.ris_client.station_information(&eva).await?;

    if response.is_none() {
        return Err(RailboardApiError {
            domain: crate::error::ErrorDomain::Ris,
            message: "No Station found".to_string(),
            error: None,
        });
    }

    let response = response.unwrap();

    {
        let response = response.clone();
        tokio::spawn(async move {
            let _ = response.insert_to_cache(&state.cache, None).await;
        });
    }

    Ok(Json(response))
}

