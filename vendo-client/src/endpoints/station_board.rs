use chrono::{DateTime, TimeZone};
use chrono_tz::{Europe::Berlin, Tz};
use reqwest::{
    header::{HeaderValue, ACCEPT, CONTENT_TYPE},
    Request, RequestBuilder,
};
use serde::Deserialize;

use crate::VendoClient;
use crate::{error::VendoError, VendoOrRequestError};

mod request;
pub use request::*;
mod response;
pub use response::*;

impl VendoClient {
    /// Get the arrival station board for a station at a specific date.
    ///
    /// The station should be given in the as the eva number (e.G. `8000105`) \
    /// or Location ID (e.G. `A=1@O=Frankfurt(Main)Hbf@X=8663785@Y=50107149@U=80@L=8000105@B=1@p=1673463547@`).
    /// If no date is provided, the current date is used.
    /// Transport types can be provided to filter the results.
    /// If no transport types are provided, all transport types are being returned.
    pub async fn station_board_arrivals(
        &self,
        station: &str,
        date: Option<DateTime<Tz>>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardArrivalsResponse, VendoOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let request = self
            .client
            .post(format!("{}{}", self.base_url, "/mob/bahnhofstafel/ankunft"))
            .station_board_request(station, date, transport_types)?;

        let response: VendoArrivalsResponse = self.client.execute(request).await?.json().await?;

        match response {
            VendoArrivalsResponse::VendoResponse(response) => Ok(*response),
            VendoArrivalsResponse::VendoError(error) => Err(VendoOrRequestError::VendoError(error)),
        }
    }

    /// Get the departure station board for a station at a specific date.
    ///
    /// The station should be given in the as the eva number (e.G. `8000105`) \
    /// or Location ID (e.G. `A=1@O=Frankfurt(Main)Hbf@X=8663785@Y=50107149@U=80@L=8000105@B=1@p=1673463547@`).
    /// If no date is provided, the current date is used.
    /// Transport types can be provided to filter the results.
    /// If no transport types are provided, all transport types are being returned.
    pub async fn station_board_departures(
        &self,
        station: &str,
        date: Option<DateTime<Tz>>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<StationBoardDeparturesResponse, VendoOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let request = self
            .client
            .post(format!("{}{}", self.base_url, "/mob/bahnhofstafel/abfahrt"))
            .station_board_request(station, date, transport_types)?;

        let response: VendoDeparturesResponse = self.client.execute(request).await?.json().await?;

        match response {
            VendoDeparturesResponse::VendoResponse(response) => Ok(*response),
            VendoDeparturesResponse::VendoError(error) => {
                Err(VendoOrRequestError::VendoError(error))
            }
        }
    }
}

trait StationBoardRequest {
    fn station_board_request(
        self,
        station: &str,
        date: Option<DateTime<Tz>>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<Request, reqwest::Error>;
}

const VENDO_STATION_BOARD_HEADER: &str = "application/x.db.vendo.mob.bahnhofstafeln.v1+json";

impl StationBoardRequest for RequestBuilder {
    fn station_board_request(
        self,
        station: &str,
        date: Option<DateTime<Tz>>,
        transport_types: Option<Vec<VendoTransportType>>,
    ) -> Result<Request, reqwest::Error> {
        let date =
            date.unwrap_or_else(|| Berlin.from_utc_datetime(&chrono::Utc::now().naive_utc()));

        let body = VendoStationBoardRequest {
            station: station.to_string(),
            date: date.format("%Y-%m-%d").to_string(),
            time: date.format("%H:%M").to_string(),
            transport_types: transport_types.unwrap_or_else(|| VendoTransportType::ALL.to_vec()),
        };

        let mut request = self
            .json(&body)
            .header("x-correlation-id", "railboard")
            .build()?;

        let headers = request.headers_mut();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(VENDO_STATION_BOARD_HEADER),
        );
        headers.insert(ACCEPT, HeaderValue::from_static(VENDO_STATION_BOARD_HEADER));

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
