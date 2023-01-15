mod response;
pub use response::*;

use crate::{IrisClient, IrisError, IrisOrRequestError};

impl IrisClient {
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
