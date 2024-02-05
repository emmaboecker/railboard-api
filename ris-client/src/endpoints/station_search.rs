pub use response::*;

use crate::{RisClient, RisOrRequestError};
use crate::request::ResponseOrRisError;

mod response;

impl RisClient {
    pub async fn station_search_by_name(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> Result<Vec<RisStationSearchElement>, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let limit = limit.unwrap_or(25);

        let url = format!(
            "{}/db/apis/ris-stations/v1/stop-places/by-name/{query}",
            self.base_url
        );

        let response: ResponseOrRisError<RisStationSearchResponse> = self
            .client
            .get(&url)
            .query(&[("limit", format!("{}", limit))])
            .header("db-api-key", &self.db_api_key)
            .header("db-client-id", &self.db_client_id)
            .send()
            .await?
            .json()
            .await?;

        match response {
            ResponseOrRisError::Response(response) => Ok(response.stop_places),
            ResponseOrRisError::Error(error) => {
                Err(RisOrRequestError::RisError(error))
            }
            ResponseOrRisError::UnauthorizedError(error) => {
                Err(RisOrRequestError::RisUnauthorizedError(error))
            }
        }
    }
}

