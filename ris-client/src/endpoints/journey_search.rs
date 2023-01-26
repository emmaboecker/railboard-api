use crate::{RisClient, RisError, RisOrRequestError, RisUnauthorizedError};

mod response;
use chrono::NaiveDate;
pub use response::*;
use serde::Deserialize;

impl RisClient {
    pub async fn journey_search(
        &self,
        category: &str,
        number: &str,
        date: Option<NaiveDate>,
    ) -> Result<RisJourneySearchResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!("{}/db/apis/ris-journeys/v1/byrelation", self.base_url);

        let mut vec = vec![
            ("category", category.to_owned()),
            ("number", number.to_owned()),
        ];

        if let Some(date) = date {
            let date = date.format("%Y-%m-%d").to_string();
            vec.push(("date", date));
        }

        let response: RisJourneySearchOrErrorResponse = self
            .client
            .get(&url)
            .query(&vec)
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
