pub use transformed::*;

use crate::{RisClient, RisOrRequestError};
use crate::request::ResponseOrRisError;
use crate::station_information::response::StationInformationResponse;

pub(crate) mod response;
mod transformed;

impl RisClient {
    #[deprecated(
        note = "the only known api key was revoked, so i cannot maintain this endpoint anymore"
    )]
    pub async fn station_information(
        &self,
        eva: &str,
    ) -> Result<Option<RisStationInformation>, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/db/apis/ris-stations/v1/stop-places/{eva}",
            self.base_url
        );

        let response: ResponseOrRisError<StationInformationResponse> = self
            .client
            .get(&url)
            .header("db-api-key", &self.db_api_key)
            .header("db-client-id", &self.db_client_id)
            .send()
            .await?
            .json()
            .await?;

        match response {
            ResponseOrRisError::Response(response) => {
                let station = response.stations.into_iter().next().map(|i| i.into());

                Ok(station)
            },
            ResponseOrRisError::Error(error) => {
                Err(RisOrRequestError::RisError(error))
            }
            ResponseOrRisError::UnauthorizedError(error) => {
                Err(RisOrRequestError::RisUnauthorizedError(error))
            }
        }
    }
}
