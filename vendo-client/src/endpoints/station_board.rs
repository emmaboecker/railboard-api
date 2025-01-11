use chrono::{DateTime, TimeZone};
use chrono_tz::{Europe::Berlin, Tz};
use reqwest::{
    header::{HeaderValue, ACCEPT, CONTENT_TYPE},
    Request, RequestBuilder,
};
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::VendoClient;
use crate::{error::VendoError, VendoOrRequestError};

mod request;
pub mod response;
mod transformed;

use crate::shared::Time;
use crate::station_board::response::{
    StationBoardArrivalsElement, StationBoardArrivalsResponse, StationBoardDeparturesElement,
    StationBoardDeparturesResponse,
};
pub use request::*;
pub use transformed::*;

impl VendoClient {
    pub async fn station_board(
        &self,
        id: &str,
        date: DateTime<Tz>,
    ) -> Result<VendoStationBoard, VendoOrRequestError> {
        let (arrivals, departures) = tokio::join!(
            self.station_board_arrivals(id, Some(date), None),
            self.station_board_departures(id, Some(date), None)
        );

        let arrivals = arrivals?;
        let departures = departures?;

        let mut trains: BTreeMap<
            String,
            (
                Option<StationBoardArrivalsElement>,
                Option<StationBoardDeparturesElement>,
            ),
        > = BTreeMap::new();

        for train in arrivals.arrivals {
            let id = train.id.clone();
            trains.entry(id).or_insert_with(|| (None, None)).0 = Some(train);
        }

        for train in departures.departures {
            let id = train.id.clone();
            trains.entry(id).or_insert_with(|| (None, None)).1 = Some(train);
        }

        let mut trains: Vec<StationBoardElement> = trains
            .into_iter()
            .map(|(id, (arrival, departure))| {
                let arrival_data = arrival.as_ref().map(|arrival| StationBoardArrival {
                    origin: arrival.origin.name.clone(),
                    time: Time {
                        scheduled: arrival.arrival_date,
                        realtime: arrival.realtime_arrival_date,
                    },
                });
                let departure_data = departure.as_ref().map(|departure| StationBoardDeparture {
                    destination: departure.destination_name.clone(),
                    time: Time {
                        scheduled: departure.departure_date,
                        realtime: departure.realtime_departure_date,
                    },
                });

                if let Some(departure) = departure {
                    StationBoardElement {
                        journey_id: id,
                        arrival: arrival_data,
                        departure: departure_data,
                        product_type: departure.product_type,
                        short_name: departure.short_name,
                        name: departure.name,
                        scheduled_platform: departure.platform,
                        realtime_platform: departure.realtime_platform,
                        notes: departure.notes.into_iter().map(|note| note.text).collect(),
                        request_station: StationBoardRequestedStation {
                            eva: departure.requested_station.eva,
                            name: departure.requested_station.name,
                            location_id: departure.requested_station.location_id,
                        },
                    }
                } else if let Some(arrival) = arrival {
                    StationBoardElement {
                        journey_id: id,
                        arrival: arrival_data,
                        departure: departure_data,

                        product_type: arrival.product_type,
                        short_name: arrival.short_name,
                        name: arrival.name,
                        scheduled_platform: arrival.platform,
                        realtime_platform: arrival.realtime_platform,
                        notes: arrival.notes.into_iter().map(|note| note.text).collect(),
                        request_station: StationBoardRequestedStation {
                            eva: arrival.requested_station.eva,
                            name: arrival.requested_station.name,
                            location_id: arrival.requested_station.location_id,
                        },
                    }
                } else {
                    panic!("Arrival and departure are both None"); // This should never happen (it is just simply not possible)
                }
            })
            .collect();

        trains.sort_by(|a, b| {
            let a_dep = a.departure.clone().map(|departure| departure.time);
            let a_arr = a.arrival.clone().map(|arrival| arrival.time);
            let b_dep = b.departure.clone().map(|departure| departure.time);
            let b_arr = b.arrival.clone().map(|arrival| arrival.time); // I need to remove these clones but I don't know how
            a_dep
                .unwrap_or_else(|| a_arr.unwrap())
                .scheduled
                .cmp(&b_dep.unwrap_or_else(|| b_arr.unwrap()).scheduled)
        });

        Ok(VendoStationBoard {
            day: date.format("%Y-%m-%d").to_string(),
            time: date.format("%H:%M").to_string(),
            id: id.to_string(),
            station_board: trains,
        })
    }

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

const VENDO_STATION_BOARD_HEADER: &str = "application/x.db.vendo.mob.bahnhofstafeln.v2+json";

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
