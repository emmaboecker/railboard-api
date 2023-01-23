use std::sync::Arc;

use axum::{routing::get, Router};
use ris_client::RisClient;

use crate::cache::RedisCache;

pub mod journey_search;

pub struct RisState {
    ris_client: Arc<RisClient>,
    cache: Arc<RedisCache>,
}

pub fn router(redis: Arc<redis::Client>, db_client_id: &str, db_api_key: &str) -> Router {
    let ris_client = Arc::new(RisClient::new(None, None, None, db_client_id, db_api_key));

    let shared_state = Arc::new(RisState {
        ris_client,
        cache: Arc::new(RedisCache::new(redis)),
    });

    Router::new()
        .route(
            "/journey_search/:category/:number",
            get(journey_search::journey_search),
        )
        .with_state(shared_state)
}
