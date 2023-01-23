use crate::{RisClient, RisError, RisOrRequestError, RisUnauthorizedError};

mod response;
pub use response::*;
use serde::Deserialize;

impl RisClient {
    pub async fn journey_search(
        &self,
        category: &str,
        number: &str,
    ) -> Result<RisJourneySearchResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/db/apis/ris-journeys/v1/byrelation?category={}&number={}",
            self.base_url, category, number
        );

        let response: RisJourneySearchOrErrorResponse = self
            .client
            .get(&url)
            .header("db-api-key", self.db_api_key.clone())
            .header("db-client-id", self.db_client_id.clone())
            .send()
            .await?
            .json()
            .await?;

        match response {
            RisJourneySearchOrErrorResponse::Response(response) => Ok(*response),
            RisJourneySearchOrErrorResponse::Error(error) => {
                Err(RisOrRequestError::RisError(error))
            }
            RisJourneySearchOrErrorResponse::UnauthorizedError(error) => {
                Err(RisOrRequestError::RisUnauthorizedError(error))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RisJourneySearchOrErrorResponse {
    Response(Box<RisJourneySearchResponse>),
    Error(RisError),
    UnauthorizedError(RisUnauthorizedError),
}
