use std::sync::Arc;

use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use iris_client::station_board::response::TimeTable;
use redis::JsonAsyncCommands;
use ris_client::{
    journey_search::RisJourneySearchResponse, station_search::RisStationSearchElement,
};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use ris_client::journey_details::RisJourneyDetails;
use ris_client::station_board::RisStationBoard;
use ris_client::station_information::RisStationInformation;
use vendo_client::journey_details::VendoJourneyDetails;
use vendo_client::station_board::VendoStationBoard;
use zugportal_client::station_board::ZugportalStationBoard;
use crate::vendo::location_search::LocationSearchCache;

#[async_trait::async_trait]
pub trait Cache: Sync + Send {
    async fn get_from_id<Rt>(&self, id: &str) -> Option<Rt>
    where
        Rt: DeserializeOwned + Sync + Send;

    async fn insert_to_cache<Rt>(
        &self,
        key: String,
        object: &Rt,
        expiration: usize,
    ) -> Result<(), CacheInsertError>
    where
        Rt: Serialize + Sync + Send;
}

#[derive(Debug, Error)]
pub enum CacheInsertError {
    #[error("Failed to insert object into Redis: {0}")]
    RedisError(#[from] redis::RedisError),
}

#[derive(Clone)]
pub struct RedisCache {
    pub redis_client: Arc<redis::Client>,
}

impl RedisCache {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }
}

#[async_trait::async_trait]
impl Cache for RedisCache {
    async fn get_from_id<Rt>(&self, id: &str) -> Option<Rt>
    where
        Rt: DeserializeOwned,
    {
        let conn = self.redis_client.get_async_connection().await;
        let mut conn = match conn {
            Ok(conn) => {
                tracing::debug!("Connection to Cache");
                conn
            }
            Err(err) => {
                tracing::error!("Error while getting connection to cache: {}", err);
                return None;
            }
        };
        let result: Result<Option<String>, redis::RedisError> = conn.json_get(id, "$").await;

        match result {
            Ok(result) => match result {
                Some(result) => {
                    let result: Result<Vec<Rt>, serde_json::Error> = serde_json::from_str(&result);
                    match result {
                        Ok(result) => {
                            tracing::debug!("Got result from cache");
                            tracing::debug!("Response cached");
                            result.into_iter().next()
                        }
                        Err(err) => {
                            tracing::error!("Error while parsing result from cache: {}", err);
                            None
                        }
                    }
                }
                None => {
                    tracing::debug!("No results in cache");
                    None
                }
            },
            Err(err) => {
                tracing::error!("Error while getting from cache: {}", err);
                None
            }
        }
    }
    async fn insert_to_cache<Rt>(
        &self,
        key: String,
        object: &Rt,
        expiration: usize,
    ) -> Result<(), CacheInsertError>
    where
        Rt: Serialize + Send + Sync,
    {
        let mut connection = self.redis_client.get_async_connection().await?;

        redis::pipe()
            .atomic()
            .json_set(&key, "$", object)?
            .ignore()
            .expire(&key, expiration)
            .ignore()
            .query_async(&mut connection)
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait CachableObject {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        information: Option<&str>,
    ) -> Result<(), CacheInsertError>;
}

#[async_trait::async_trait]
impl CachableObject for VendoStationBoard {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("vendo.station-board.{}.{}.{}", self.id, self.day, self.time);

        cache.insert_to_cache(key, self, 90).await
    }
}

#[async_trait::async_trait]
impl CachableObject for LocationSearchCache {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("vendo.location-search.{}", self.query);

        cache.insert_to_cache(key, self, 60 * 60 * 24 * 7).await
    }
}

#[async_trait::async_trait]
impl CachableObject for VendoJourneyDetails {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("vendo.journey-details.{}", self.journey_id);

        cache.insert_to_cache(key, self, 90).await
    }
}

#[async_trait::async_trait]
impl CachableObject for (TimeTable, String, String, String) {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("iris.station-board.plan.{}.{}.{}", self.1, self.2, self.3);

        cache.insert_to_cache(key, &self.0, 180).await
    }
}

#[async_trait::async_trait]
impl CachableObject for (TimeTable, String) {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("iris.station-board.realtime.{}", self.1);

        cache.insert_to_cache(key, &self.0, 30).await
    }
}

#[async_trait::async_trait]
impl CachableObject for (String, String, RisJourneySearchResponse) {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!(
            "ris.journey-search.{}.{}.{}",
            self.0,
            self.1,
            self.2
                .journeys
                .first()
                .map(|first| first.date.clone())
                .unwrap_or_else(|| Berlin
                    .from_utc_datetime(&chrono::Utc::now().naive_utc())
                    .format("%Y-%m-%d")
                    .to_string())
        );

        cache.insert_to_cache(key, &self.2.journeys, 600).await
    }
}

#[async_trait::async_trait]
impl CachableObject for RisJourneyDetails {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("ris.journey-details.{}", self.id);

        cache.insert_to_cache(key, &self, 90).await
    }
}

#[async_trait::async_trait]
impl CachableObject for RisStationBoard {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!(
            "ris.station-board.{}.{}.{}",
            self.eva,
            self.time_start.naive_utc().format("%Y-%m-%dT%H:%M"),
            self.time_end.naive_utc().format("%Y-%m-%dT%H:%M")
        );

        cache.insert_to_cache(key, &self, 180).await
    }
}

#[async_trait::async_trait]
impl CachableObject for ZugportalStationBoard {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!(
            "zugportal.station-board.{}.{}.{}",
            self.eva,
            self.time_start.naive_utc().format("%Y-%m-%dT%H:%M"),
            self.time_end.naive_utc().format("%Y-%m-%dT%H:%M")
        );

        cache.insert_to_cache(key, &self, 180).await
    }
}

#[async_trait::async_trait]
impl CachableObject for RisStationInformation {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        _information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("ris.station-information.{}", self.eva);

        cache.insert_to_cache(key, &self, 180).await
    }
}

#[async_trait::async_trait]
impl CachableObject for Vec<RisStationSearchElement> {
    async fn insert_to_cache<C: Cache>(
        &self,
        cache: &C,
        information: Option<&str>,
    ) -> Result<(), CacheInsertError> {
        let key = format!("ris.station-search-by-name.{}", information.unwrap_or(""));

        cache.insert_to_cache(key, &self, 60 * 60).await
    }
}
