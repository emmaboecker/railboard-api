mod response;
use crate::{RisClient, RisOrRequestError, ZugportalError};
use chrono::DateTime;
use chrono_tz::Tz;
pub use response::*;
use serde::Deserialize;

// This endpoint uses the "zugportal" api that the "Zugportal" App uses, and its basically ris so I didnt feel like making another package for it

impl RisClient {
    pub async fn station_board_departures(
        &self,
        eva: &str,
        time_start: Option<DateTime<Tz>>,
        time_end: Option<DateTime<Tz>>,
    ) -> Result<StationBoardResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "https://zugportal.de/@prd/zupo-travel-information/api/public/ri/board/departure/{eva}"
        );

        station_board(self, url, time_start, time_end).await
    }

    pub async fn station_board_arrivals(
        &self,
        eva: &str,
        time_start: Option<DateTime<Tz>>,
        time_end: Option<DateTime<Tz>>,
    ) -> Result<StationBoardResponse, RisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "https://zugportal.de/@prd/zupo-travel-information/api/public/ri/board/arrival/{eva}"
        );

        station_board(self, url, time_start, time_end).await
    }
}

async fn station_board(
    client: &RisClient,
    url: String,
    time_start: Option<DateTime<Tz>>,
    time_end: Option<DateTime<Tz>>,
) -> Result<StationBoardResponse, RisOrRequestError> {
    let mut query = vec![("expandTimeFrame", "TIME_START".to_owned())]; // todo: find out what this means because I have no idea

    if let Some(time_start) = time_start {
        let time = time_start.to_rfc3339();
        println!("{time}");
        query.push(("timeStart", time))
    }

    if let Some(time_end) = time_end {
        query.push(("timeEnd", time_end.to_rfc3339()))
    }

    let response: RisStationBoardOrErrorResponse = client
        .client
        .get(&url)
        .query(&query)
        .send()
        .await?
        .json()
        .await?;

    match response {
        RisStationBoardOrErrorResponse::Response(response) => Ok(*response),
        RisStationBoardOrErrorResponse::Error(error) => {
            Err(RisOrRequestError::ZugportalError(error))
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RisStationBoardOrErrorResponse {
    Response(Box<StationBoardResponse>),
    Error(ZugportalError),
}
