use reqwest::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::Deserialize;
use urlencoding::encode;

pub use transformed::*;

use crate::journey_details::response::JourneyDetailsResponse;
use crate::shared::Time;
use crate::{VendoClient, VendoError, VendoOrRequestError};

pub mod response;
mod transformed;

const VENDO_JOURNEY_DETAILS_HEADER: &str = "application/x.db.vendo.mob.zuglauf.v2+json";

impl VendoClient {
    /// Get journey details for a specific journey.
    ///
    /// The ID has to be a Vendo ID e.G. \
    /// `2|#VN#1#ST#1673463547#PI#0#ZI#166635#TA#0#DA#150123#1S#8006132#1T#1415#LS#8000105#LT#1514#PU#80#RT#1#CA#RB#ZE#15519#ZB#RB 15519#PC#3#FR#8006132#FT#1415#TO#8000105#TT#1514#`
    pub async fn journey_details(
        &self,
        id: &str,
    ) -> Result<VendoJourneyDetails, VendoOrRequestError> {
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
            VendoJourneyDetailsResponse::VendoResponse(response) => {
                let mapped = VendoJourneyDetails {
                    short_name: response.short_name,
                    name: response.name,
                    long_name: response.long_name,
                    destination: response.destination,

                    journey_id: id.to_string(),

                    stops: response
                        .stops
                        .into_iter()
                        .map(|stop| VendoStop {
                            name: stop.stop_details.name,
                            eva: stop.stop_details.eva,
                            position: PolylinePosition {
                                longitude: stop.stop_details.position.longitude,
                                latitude: stop.stop_details.position.latitude,
                            },
                            arrival: stop.arrival.map(|arrival| Time {
                                scheduled: arrival,
                                realtime: stop.realtime_arrival,
                            }),
                            departure: stop.departure.map(|departure| Time {
                                scheduled: departure,
                                realtime: stop.realtime_departure,
                            }),
                            platform: stop.platform,
                            realtime_platform: stop.realtime_platform,
                            notes: stop.notes.into_iter().map(|note| note.text).collect(),
                            him_notices: stop
                                .him_notices
                                .into_iter()
                                .map(|from| from.into())
                                .collect(),
                            attributes: stop
                                .attributes
                                .into_iter()
                                .map(|from| from.into())
                                .collect(),
                            service_note: stop.service_note.map(|service| service.into()),
                        })
                        .collect(),

                    transport_number: response.transport_number,
                    product_type: response.product_type,
                    notes: response.notes.into_iter().map(|note| note.text).collect(),
                    him_notices: response
                        .him_notices
                        .into_iter()
                        .map(|from| from.into())
                        .collect(),
                    attributes: response
                        .attributes
                        .into_iter()
                        .map(|from| from.into())
                        .collect(),
                    schedule: VendoTrainSchedule {
                        regular_schedule: response.schedule.regular_schedule,
                        days_of_operation: response.schedule.days_of_operation,
                    },
                    journey_day: response.journey_day,

                    polyline: response.polyline_group.map(|group| {
                        group
                            .polyline_desc
                            .map(|desc| {
                                desc.first()
                                    .unwrap()
                                    .coordinates
                                    .iter()
                                    .map(|point| PolylinePosition {
                                        longitude: point.longitude,
                                        latitude: point.latitude,
                                    })
                                    .collect()
                            })
                            .unwrap_or_default()
                    }),
                };

                Ok(mapped)
            }
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
