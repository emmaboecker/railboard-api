use redis::JsonAsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::vendo::{
    journey_details::JoruneyDetails, location_search::LocationSearchCache,
    station_board::StationBoard,
};

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

pub struct RedisCache {
    pub redis_client: redis::Client,
}

impl RedisCache {
    pub fn new(redis_client: redis::Client) -> Self {
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
    async fn insert_to_cache<C: Cache>(&self, cache: &C) -> Result<(), CacheInsertError>;
}

#[async_trait::async_trait]
impl CachableObject for StationBoard {
    async fn insert_to_cache<C: Cache>(&self, cache: &C) -> Result<(), CacheInsertError> {
        let key = format!("station-board.{}.{}.{}", self.id, self.day, self.time);

        cache.insert_to_cache(key, self, 90).await
    }
}

#[async_trait::async_trait]
impl CachableObject for LocationSearchCache {
    async fn insert_to_cache<C: Cache>(&self, cache: &C) -> Result<(), CacheInsertError> {
        let key = format!("location-search.{}", self.query);

        cache.insert_to_cache(key, self, 60 * 60 * 24 * 7).await
    }
}

#[async_trait::async_trait]
impl CachableObject for JoruneyDetails {
    async fn insert_to_cache<C: Cache>(&self, cache: &C) -> Result<(), CacheInsertError> {
        let key = format!("journey-details.{}", self.journey_id);

        cache.insert_to_cache(key, self, 90).await
    }
}
