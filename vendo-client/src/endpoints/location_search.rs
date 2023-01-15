use crate::{VendoClient, VendoError, VendoOrRequestError};

mod request;
pub use request::*;
mod response;
use reqwest::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
pub use response::*;
use serde::Deserialize;

const VENDO_LOCATION_SEARCH_HEADER: &str = "application/x.db.vendo.mob.location.v3+json";

impl VendoClient {
    /// Search for a location (e.G. Station).
    pub async fn location_search(
        &self,
        query: String,
        location_types: Option<Vec<String>>,
    ) -> Result<Vec<LocationSearchResult>, VendoOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let location_types = location_types.unwrap_or_default();

        let request = request::LocationSearchRequest {
            search_term: query,
            location_types,
        };

        let mut request = self
            .client
            .post(format!("{}/mob/location/search/", self.base_url))
            .json(&request)
            .build()?;

        let headers = request.headers_mut();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(VENDO_LOCATION_SEARCH_HEADER),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static(VENDO_LOCATION_SEARCH_HEADER),
        );

        headers.insert("x-correlation-id", HeaderValue::from_static("railboard"));

        let response = self.client.execute(request).await?.json().await?;

        match response {
            VendoLocationSearchResponse::VendoResponse(response) => Ok(response),
            VendoLocationSearchResponse::VendoError(error) => {
                Err(VendoOrRequestError::VendoError(error))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum VendoLocationSearchResponse {
    VendoResponse(Vec<LocationSearchResult>),
    VendoError(VendoError),
}
