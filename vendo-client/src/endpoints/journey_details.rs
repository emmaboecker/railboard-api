mod response;
use reqwest::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
pub use response::*;
use serde::Deserialize;
use urlencoding::encode;

use crate::{VendoClient, VendoError, VendoOrRequestError};

const VENDO_JOURNEY_DETAILS_HEADER: &str = "application/x.db.vendo.mob.zuglauf.v1+json";

impl VendoClient {
    /// Get journey details for a specific journey.
    /// 
    /// The ID has to be a Vendo ID e.G. \
    /// `2|#VN#1#ST#1673463547#PI#0#ZI#166635#TA#0#DA#150123#1S#8006132#1T#1415#LS#8000105#LT#1514#PU#80#RT#1#CA#RB#ZE#15519#ZB#RB 15519#PC#3#FR#8006132#FT#1415#TO#8000105#TT#1514#`
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
