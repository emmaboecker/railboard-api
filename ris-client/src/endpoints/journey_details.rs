use crate::{RisClient, RisError, RisOrRequestError, RisUnauthorizedError};

mod response;
pub use response::*;
use serde::Deserialize;

impl RisClient {
    pub async fn journey_details(
        &self,
        id: &str,
    ) -> Result<JourneyDetailsResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/db/apis/ris-journeys/v1/eventbased/{}",
            self.base_url, id,
        );

        let response: RisJourneyDetailsOrErrorResponse = self
            .client
            .get(&url)
            .header("db-api-key", self.db_api_key.clone())
            .header("db-client-id", self.db_client_id.clone())
            .send()
            .await?
            .json()
            .await?;

        match response {
            RisJourneyDetailsOrErrorResponse::Response(response) => Ok(*response),
            RisJourneyDetailsOrErrorResponse::Error(error) => {
                Err(RisOrRequestError::RisError(error))
            }
            RisJourneyDetailsOrErrorResponse::UnauthorizedError(error) => {
                Err(RisOrRequestError::RisUnauthorizedError(error))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RisJourneyDetailsOrErrorResponse {
    Response(Box<JourneyDetailsResponse>),
    Error(RisError),
    UnauthorizedError(RisUnauthorizedError),
}
