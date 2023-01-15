mod response;
pub use response::*;

use crate::{IrisClient, IrisError, IrisOrRequestError};

impl IrisClient {
    /// Get all realtime information IRIS currently has for a specific station. 
    /// 
    /// Takes the eva number of the station e.G. `8000105` for Frankfurt(Main)Hbf.
    pub async fn realtime_station_board(
        &self,
        eva: String,
    ) -> Result<TimeTable, IrisOrRequestError> {
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

        println!("{}", response);

        let response = serde_xml_rs::from_str(&response)?;

        Ok(response)
    }

    /// Get all planned information IRIS has for a specific station at the specified date + hour.
    /// 
    /// From experience IRIS does not have any more planned data than the current day + maybe a bit of the early hours of the next day.
    /// 
    /// Takes the eva number of the station e.G. `8000105` for Frankfurt(Main)Hbf. \
    /// the date in the format `YYMMDD` \
    /// and the hour in the format `HH`.
    pub async fn planned_station_board(
        &self,
        eva: String,
        date: String,
        hour: String,
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

        println!("{}", response);

        let response = serde_xml_rs::from_str(&response)?;

        Ok(response)
    }
}
