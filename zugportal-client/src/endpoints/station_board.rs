mod response;

use std::collections::BTreeMap;

use chrono::DateTime;
use chrono_tz::Tz;
use serde::Deserialize;

pub use transformed::*;

use crate::{ZugportalClient};
use crate::error::{ZugportalError, ZugportalOrRequestError};
use crate::helpers::name_from_administation_code;
use crate::station_board::response::{StationBoardItem, StationBoardResponse};

mod transformed;

impl ZugportalClient {
    #[deprecated]
    pub async fn station_board(
        &self,
        eva: &str,
        time_start: Option<DateTime<Tz>>,
        time_end: Option<DateTime<Tz>>,
    ) -> Result<ZugportalStationBoard, ZugportalOrRequestError> {
        let (arrivals, departures) = tokio::join!(
            self.station_board_arrivals(eva, time_start, time_end),
            self.station_board_departures(eva, time_start, time_end)
        );

        let arrivals = arrivals?;
        let departures = departures?;

        let mut trains: BTreeMap<String, (Option<StationBoardItem>, Option<StationBoardItem>)> =
            BTreeMap::new();

        for train in arrivals.items {
            let id = train.train.journey_id.to_owned();
            trains.entry(id).or_insert_with(|| (None, None)).0 = Some(train);
        }

        for train in departures.items {
            let id = train.train.journey_id.to_owned();
            trains.entry(id).or_insert_with(|| (None, None)).1 = Some(train);
        }

        if departures.station_name.is_none() {
            return Err(ZugportalOrRequestError::NotFoundError);
        }

        Ok(ZugportalStationBoard {
            eva: departures.eva_no.unwrap_or(eva.to_string()),
            name: departures.station_name.unwrap_or_default(),
            time_start: departures.time_start,
            time_end: departures.time_end,
            items: trains
                .into_iter()
                .map(|(id, (arrival, departure))| {
                    let departure_arrival = departure
                        .clone()
                        .unwrap_or_else(|| arrival.clone().unwrap());

                    let scheduled_platform = if departure_arrival.platform.is_empty() {
                        None
                    } else {
                        Some(departure_arrival.platform)
                    };
                    let realtime_platform = if departure_arrival.platform_predicted.is_empty() {
                        None
                    } else {
                        Some(departure_arrival.platform_predicted)
                    };

                    ZugportalStationBoardItem {
                        journey_id: id,
                        station_eva: departure_arrival.station.eva_no.clone(),
                        station_name: departure_arrival.station.name.clone(),
                        cancelled: departure_arrival.canceled || departure_arrival.station.canceled,
                        category: departure_arrival.train.category,
                        train_type: departure_arrival.train.r#type,
                        train_number: departure_arrival.train.no,
                        line_indicator: departure_arrival.train.line_name,
                        departure: departure.as_ref().map(|departure| ZugportalDepartureArrival {
                            delay: departure.diff,
                            time_realtime: departure.time_predicted,
                            time_scheduled: departure.time,
                            time_type: departure.time_type.clone(),
                        }),
                        arrival: arrival.as_ref().map(|arrival| ZugportalDepartureArrival {
                            delay: arrival.diff,
                            time_realtime: arrival.time_predicted,
                            time_scheduled: arrival.time,
                            time_type: arrival.time_type.clone(),
                        }),
                        destination_eva: departure
                            .as_ref()
                            .and_then(|departure| {
                                departure
                                    .destination
                                    .as_ref()
                                    .map(|destination| destination.eva_no.clone())
                            })
                            .unwrap_or(departure_arrival.station.eva_no.clone()),
                        destination_name: departure
                            .and_then(|departure| {
                                departure.destination.map(|destination| destination.name)
                            })
                            .unwrap_or(departure_arrival.station.name.clone()),
                        origin_eva: arrival
                            .as_ref()
                            .and_then(|arrival| {
                                arrival.origin.as_ref().map(|origin| origin.eva_no.clone())
                            })
                            .unwrap_or(departure_arrival.station.eva_no),
                        origin_name: arrival
                            .as_ref()
                            .and_then(|arrival| {
                                arrival.origin.as_ref().map(|origin| origin.name.clone())
                            })
                            .unwrap_or(departure_arrival.station.name),
                        platform_scheduled: scheduled_platform,
                        platform_realtime: realtime_platform,
                        administation: ZugportalStationBoardItemAdministration {
                            id: departure_arrival.administration.id,
                            operator_code: departure_arrival.administration.operator_code,
                            operator_name: String::from(
                                name_from_administation_code(
                                    &departure_arrival.administration.operator_name,
                                )
                                    .unwrap_or(&departure_arrival.administration.operator_name),
                            ),
                            ris_operator_name: departure_arrival.administration.operator_name,
                        },
                    }
                })
                .collect(),
        })
    }

    pub async fn station_board_departures(
        &self,
        eva: &str,
        time_start: Option<DateTime<Tz>>,
        time_end: Option<DateTime<Tz>>,
    ) -> Result<StationBoardResponse, ZugportalOrRequestError> {
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
    ) -> Result<StationBoardResponse, ZugportalOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let url = format!(
            "{}/@prd/zupo-travel-information/api/public/ri/board/arrival/{eva}",
            self.base_url
        );

        station_board(self, url, time_start, time_end).await
    }
}

async fn station_board(
    client: &ZugportalClient,
    url: String,
    time_start: Option<DateTime<Tz>>,
    time_end: Option<DateTime<Tz>>,
) -> Result<StationBoardResponse, ZugportalOrRequestError> {
    let mut query = vec![("sortBy", "TIME_SCHEDULE".to_owned()), ("includeStationGroup", "true".to_owned())];

    if let Some(time_start) = time_start {
        query.push(("timeStart", time_start.to_rfc3339()))
    }

    if let Some(time_end) = time_end {
        query.push(("timeEnd", time_end.to_rfc3339()))
    }

    let response: ZugportalStationBoardOrErrorResponse = client
        .client
        .get(&url)
        .query(&query)
        .send()
        .await?
        .json()
        .await?;

    match response {
        ZugportalStationBoardOrErrorResponse::Response(response) => Ok(*response),
        ZugportalStationBoardOrErrorResponse::Error(error) => {
            Err(ZugportalOrRequestError::ZugportalError(error))
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ZugportalStationBoardOrErrorResponse {
    Response(Box<StationBoardResponse>),
    Error(ZugportalError),
}
