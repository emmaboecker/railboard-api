use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use chrono_tz::Europe::Berlin;
use reqwest::{
    header::{HeaderValue, ACCEPT, CONTENT_TYPE},
    Request, RequestBuilder,
};
use serde::Deserialize;
use thiserror::Error;

use crate::client::errors::VendoError;

mod request;
pub use request::*;
mod response;
pub use response::*;

use super::VendoClient;

#[async_trait]
pub trait StationBoard {
    async fn station_board_arrivals(
        &self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardArrivalsResponse, StationBoardError>;

    async fn station_board_departures(
        &self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardDeparturesResponse, StationBoardError>;
}

#[async_trait]
impl StationBoard for VendoClient {
    async fn station_board_arrivals(
        &self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardArrivalsResponse, StationBoardError> {
        let _permit = self.semaphore.acquire().await;

        let request = self
            .client
            .post(format!("{}{}", self.base_url, "/mob/bahnhofstafel/ankunft"))
            .station_board_request(station, date, transport_types)?;

        let reponse: VendoArrivalsResponse = self.client.execute(request).await?.json().await?;

        match reponse {
            VendoArrivalsResponse::VendoResponse(response) => Ok(*response),
            VendoArrivalsResponse::VendoError(error) => Err(StationBoardError::VendoError(error)),
        }
    }

    async fn station_board_departures(
        &self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardDeparturesResponse, StationBoardError> {
        let _permit = self.semaphore.acquire().await;

        let request = self
            .client
            .post(format!("{}{}", self.base_url, "/mob/bahnhofstafel/abfahrt"))
            .station_board_request(station, date, transport_types)?;

        let reponse: VendoDeparturesResponse = self.client.execute(request).await?.json().await?;

        match reponse {
            VendoDeparturesResponse::VendoResponse(response) => Ok(*response),
            VendoDeparturesResponse::VendoError(error) => Err(StationBoardError::VendoError(error)),
        }
    }
}

trait StationBoardRequest {
    fn station_board_request(
        self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<Request, reqwest::Error>;
}

impl StationBoardRequest for RequestBuilder {
    fn station_board_request(
        self,
        station: &str,
        date: Option<NaiveDateTime>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<Request, reqwest::Error> {
        let current_date = Utc::now().with_timezone(&Berlin).naive_utc();
        let date = date.unwrap_or(current_date);

        let body = VendoStationBoardRequest {
            station: station.to_string(),
            date: date.format("%Y-%m-%d").to_string(),
            time: date.format("%H:%M").to_string(),
            transport_types: transport_types.unwrap_or(VendoTransportType::all()),
        };

        let mut request = self
            .json(&body)
            .header("x-correlation-id", "railboard")
            .build()?;

        let headers = request.headers_mut();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x.db.vendo.mob.bahnhofstafeln.v1+json"),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/x.db.vendo.mob.bahnhofstafeln.v1+json"),
        );

        Ok(request)
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum VendoArrivalsResponse {
    VendoResponse(Box<StationBoardArrivalsResponse>),
    VendoError(VendoError),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum VendoDeparturesResponse {
    VendoResponse(Box<StationBoardDeparturesResponse>),
    VendoError(VendoError),
}

#[derive(Error, Debug)]
pub enum StationBoardError {
    #[error("Vendo returned an error.")]
    VendoError(#[from] VendoError),
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
