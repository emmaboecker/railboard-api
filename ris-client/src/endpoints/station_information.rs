mod response;
pub use response::*;

use crate::{RisClient, RisOrRequestError};

impl RisClient {
    pub async fn station_information(
        &self,
        eva: &str,
    ) -> Result<StationInformationResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/db/apis/ris-stations/v1/stop-places/{eva}",
            self.base_url
        );

        let response = self
            .client
            .get(&url)
            .header("db-api-key", &self.db_api_key)
            .header("db-client-id", &self.db_client_id)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
