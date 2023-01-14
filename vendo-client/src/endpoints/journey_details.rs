mod response;
use reqwest::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
pub use response::*;
use serde::Deserialize;
use urlencoding::encode;

use crate::{VendoClient, VendoError, VendoOrRequestError};

const VENDO_JOURNEY_DETAILS_HEADER: &str = "application/x.db.vendo.mob.zuglauf.v1+json";

impl VendoClient {
    pub async fn journey_details(
        &self,
        id: &str,
    ) -> Result<JourneyDetailsResponse, VendoOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let response: VendoJourneyDetailsResponse = self
            .client
            .get(format!("{}/mob/zuglauf/{}", self.base_url, encode(id)))
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static(VENDO_JOURNEY_DETAILS_HEADER),
            )
            .header(
                ACCEPT,
                HeaderValue::from_static(VENDO_JOURNEY_DETAILS_HEADER),
            )
            .header("x-correlation-id", "railboard")
            .send()
            .await?
            .json()
            .await?;

        match response {
            VendoJourneyDetailsResponse::VendoResponse(response) => Ok(*response),
            VendoJourneyDetailsResponse::VendoError(error) => {
                Err(VendoOrRequestError::VendoError(error))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum VendoJourneyDetailsResponse {
    VendoResponse(Box<JourneyDetailsResponse>),
    VendoError(VendoError),
}
