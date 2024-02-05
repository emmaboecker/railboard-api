pub mod response;
mod r#return;

use chrono::{DateTime, Duration, TimeZone, Timelike};
use chrono_tz::{Europe::Berlin, Tz};

pub use r#return::*;

use response::*;

use crate::{IrisClient, IrisError, IrisOrRequestError};

impl IrisClient {
    /// Fetches all planned information IRIS has for a specific station at the specified time frame \
    /// and the realtime information IRIS currently has for the specified station and combines them into a \
    /// better format that is easier to work with.
    pub async fn station_board(
        &self,
        eva: &str,
        date: Option<DateTime<Tz>>,
        lookahead: Option<u32>,
        lookbehind: Option<u32>,
    ) -> Result<IrisStationBoard, IrisOrRequestError> {
        let date =
            date.unwrap_or_else(|| Berlin.from_utc_datetime(&chrono::Utc::now().naive_utc()));

        let lookbehind = lookbehind.unwrap_or(20);
        let lookahead = lookahead.unwrap_or(180);

        let lookbehind = date - chrono::Duration::minutes(lookbehind as i64);
        let lookahead = date + chrono::Duration::minutes(lookahead as i64);

        let mut dates = Vec::new();

        for current_date in DateRange(lookbehind, lookahead) {
            dates.push(current_date);
        }

        let (realtime, timetables) = tokio::join!(
            self.realtime_station_board(eva),
            futures::future::join_all(dates.iter().map(|date| async move {
                self.planned_station_board(
                    eva,
                    &date.format("%y%m%d").to_string(),
                    &date.format("%H").to_string(),
                )
                .await
            }))
        );

        let realtime = realtime?;
        let timetables = timetables
            .into_iter()
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>();

        let disruptions = realtime
            .disruptions
            .into_iter()
            .map(|message| message.into())
            .collect::<Vec<r#return::Message>>();

        let mut stops = Vec::new();

        for timetable in timetables {
            for stop in timetable.stops {
                let realtime = realtime
                    .stops
                    .iter()
                    .find(|realtime_stop| realtime_stop.id == stop.id);
                stops.push(from_iris_timetable(
                    eva,
                    &timetable.station_name,
                    stop,
                    realtime.map(|realtime| realtime.to_owned()),
                ));
            }
        }

        Ok(IrisStationBoard {
            station_name: realtime.station_name,
            station_eva: String::from(eva),
            disruptions,
            stops,
        })
    }

    /// Get all realtime information IRIS currently has for a specific station.
    ///
    /// **Consider using [`station_board`](IrisClient::station_board) instead.** \
    ///
    /// Takes the eva number of the station e.G. `8000105` for Frankfurt(Main)Hbf.
    pub async fn realtime_station_board(&self, eva: &str) -> Result<TimeTable, IrisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let response = self
            .client
            .get(format!("{}/iris-tts/timetable/fchg/{}", self.base_url, eva))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(IrisOrRequestError::IrisError(IrisError));
        }

        let response: String = response.text().await?;

        let response = serde_xml_rs::from_str(&response)?;

        Ok(response)
    }

    /// Get all planned information IRIS has for a specific station at the specified date + hour.
    ///
    /// From experience IRIS does not have more planned data than the current day + maybe a bit of the early hours of the next day.
    ///
    /// **Consider using [`station_board`](IrisClient::station_board) instead.** \
    ///
    /// Takes the eva number of the station e.G. `8000105` for Frankfurt(Main)Hbf. \
    /// the date in the format `YYMMDD` \
    /// and the hour in the format `HH`.
    pub async fn planned_station_board(
        &self,
        eva: &str,
        date: &str,
        hour: &str,
    ) -> Result<TimeTable, IrisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let response = self
            .client
            .get(format!(
                "{}/iris-tts/timetable/plan/{}/{}/{}",
                self.base_url, eva, date, hour
            ))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(IrisOrRequestError::IrisError(IrisError));
        }

        let response: String = response.text().await?;

        Ok(serde_xml_rs::from_str(&response)?)
    }
}

struct DateRange(DateTime<Tz>, DateTime<Tz>);

impl Iterator for DateRange {
    type Item = DateTime<Tz>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 || self.0.hour() == self.1.hour() {
            let next = self.0 + Duration::hours(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
