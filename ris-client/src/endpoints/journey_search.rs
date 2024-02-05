use chrono::NaiveDate;

pub use response::*;

use crate::{RisClient, RisOrRequestError};
use crate::request::ResponseOrRisError;

mod response;

impl RisClient {
    pub async fn journey_search(
        &self,
        category: &str,
        number: &str,
        date: Option<NaiveDate>,
    ) -> Result<RisJourneySearchResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!("{}/db/apis/ris-journeys/v1/byrelation", self.base_url);

        let mut query = vec![
            ("category", category.to_owned()),
            ("number", number.to_owned()),
        ];

        if let Some(date) = date {
            let date = date.format("%Y-%m-%d").to_string();
            query.push(("date", date));
        }

        let response: ResponseOrRisError<RisJourneySearchResponse> = self
            .client
            .get(&url)
            .query(&query)
            .header("db-api-key", self.db_api_key.clone())
            .header("db-client-id", self.db_client_id.clone())
            .send()
            .await?
            .json()
            .await?;

        match response {
            ResponseOrRisError::Response(response) => Ok(*response),
            ResponseOrRisError::Error(error) => {
                Err(RisOrRequestError::RisError(error))
            }
            ResponseOrRisError::UnauthorizedError(error) => {
                Err(RisOrRequestError::RisUnauthorizedError(error))
            }
        }
    }
}