use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    cache::{CachableObject, Cache},
    error::{RailboardApiError, RailboardResult},
};

use super::RisState;

#[utoipa::path(
    get,
    path = "/ris/v1/station/{eva}",
    params(
        ("eva" = String, Path, description = "The Eva Number of the Station you are requesting"),
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Station Information", body = StationInformation),
        (status = 400, description = "The Error returned by the Ris, will be the Ris Domain with UnderlyingApiError Variant 3, 4 or none if there was no Station found", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn station_information(
    Path(eva): Path<String>,
    State(state): State<Arc<RisState>>,
) -> RailboardResult<Json<StationInformation>> {
    if let Some(cached) = state
        .cache
        .get_from_id(&format!("ris.station-information.{}", &eva))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state.ris_client.station_information(&eva).await?;

    let first = response.stations.into_iter().next();

    if first.is_none() {
        return Err(RailboardApiError {
            domain: crate::error::ErrorDomain::Ris,
            message: "No Station found".to_string(),
            error: None,
        });
    }

    let response = first.unwrap();

    let response: StationInformation = response.into();

    {
        let response = response.clone();
        tokio::spawn(async move {
            let _ = response.insert_to_cache(state.cache.clone().as_ref(), None).await;
        });
    }

    Ok(Json(response))
}

impl From<ris_client::station_information::StationInformation> for StationInformation {
    fn from(value: ris_client::station_information::StationInformation) -> Self {
        Self {
            eva: value.eva_number,
            names: StationNameContent {
                name_long: value.names.de.name_long,
                speech_long: value.names.de.speech_long,
                speech_short: value.names.de.speech_short,
            },
            station_id: value.station_id,
            available_transports: value.available_transports,
            transport_associations: value.transport_associations,
            country_code: value.country_code,
            state: value.state,
            municipality_key: value.municipality_key,
            time_zone: value.time_zone,
            metropolis: value.metropolis.map(|m| m.de),
            position: Position {
                longitude: value.position.longitude,
                latitude: value.position.latitude,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationInformation {
    pub eva: String,
    #[schema(nullable)]
    pub station_id: Option<String>,
    pub names: StationNameContent,
    pub metropolis: Option<String>,
    pub available_transports: Vec<String>,
    pub transport_associations: Vec<String>,
    pub country_code: String,
    pub state: String,
    pub municipality_key: String,
    pub time_zone: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StationNameContent {
    pub name_long: String,
    #[schema(nullable)]
    pub speech_long: Option<String>,
    #[schema(nullable)]
    pub speech_short: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
}
