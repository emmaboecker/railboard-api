use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

use crate::vendo::{
    journey_details::JoruneyDetails, location_search::LocationSearchCache,
    station_board::StationBoard,
};

pub struct RedisCache {
    pub redis_client: redis::Client,
}

impl RedisCache {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }
}

#[async_trait::async_trait]
pub trait CachableObject {
    async fn insert_to_cache(&self, cache: &RedisCache) -> Result<(), redis::RedisError>;

    async fn get_from_id<Rt>(id: &str, cache: &RedisCache) -> Option<Rt>
    where
        Rt: Serialize + DeserializeOwned,
    {
        let conn = cache.redis_client.get_async_connection().await;
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
        let result: Result<Option<String>, redis::RedisError> = conn.get(id).await;

        match result {
            Ok(result) => match result {
                Some(result) => {
                    let result = serde_json::from_str(&result);
                    match result {
                        Ok(result) => {
                            tracing::debug!("Got result from cache");
                            tracing::debug!("Response cached");
                            Some(result)
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
}

#[async_trait::async_trait]
impl CachableObject for StationBoard {
    async fn insert_to_cache(&self, cache: &RedisCache) -> Result<(), redis::RedisError> {
        let serialized = if let Ok(serialized) = serde_json::to_string(&self) {
            serialized
        } else {
            tracing::error!("Error while serializing station board");
            return Ok(());
        };

        let mut connection = cache.redis_client.get_async_connection().await?;
        let _: () = connection
            .set_ex(
                &format!("station-board.{}.{}.{}", self.id, self.day, self.time),
                serialized,
                20,
            )
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl CachableObject for LocationSearchCache {
    async fn insert_to_cache(&self, cache: &RedisCache) -> Result<(), redis::RedisError> {
        let serialized = match serde_json::to_string(&self) {
            Ok(serialized) => serialized,
            Err(err) => {
                tracing::error!("Error while serializing location search: {}", err);
                return Ok(());
            }
        };

        let mut connection = cache.redis_client.get_async_connection().await?;
        let _: () = connection
            .set_ex(
                &format!("location-search.{}", self.query),
                serialized,
                60 * 60 * 24 * 7,
            )
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl CachableObject for JoruneyDetails {
    async fn insert_to_cache(&self, cache: &RedisCache) -> Result<(), redis::RedisError> {
        let serialized = match serde_json::to_string(&self) {
            Ok(serialized) => serialized,
            Err(err) => {
                tracing::error!("Error while serializing journey details: {}", err);
                return Ok(());
            }
        };

        let mut connection = cache.redis_client.get_async_connection().await?;
        let _: () = connection
            .set_ex(
                &format!("journey-details.{}", self.journey_id),
                serialized,
                60,
            )
            .await?;
        Ok(())
    }
}
