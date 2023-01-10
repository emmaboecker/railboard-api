use axum::{extract::Path, Json};
use vendo_client::location_search::LocationSearchError;

use crate::{
    error::{ErrorDomain, RailboardApiError, RailboardResult},
    VENDO_CLIENT,
};

pub async fn location_search(
    Path(query): Path<String>,
) -> RailboardResult<Json<Vec<vendo_client::location_search::LocationSearchResult>>> {
    let result = VENDO_CLIENT.location_search(query, None).await?;

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
