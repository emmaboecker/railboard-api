mod response;
pub use response::*;

use crate::{RisClient, RisOrRequestError};

impl RisClient {
    pub async fn station_search_by_name(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> Result<RisStationSearchResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let limit = limit.unwrap_or(25);

        let url = format!(
            "{}/db/apis/ris-stations/v1/stop-places/by-name/{query}",
            self.base_url
        );

        let response = self
            .client
            .get(&url)
            .query(&[("limit", format!("{}", limit))])
            .header("db-api-key", &self.db_api_key)
            .header("db-client-id", &self.db_client_id)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
