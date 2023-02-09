use std::sync::Arc;

use axum::{routing::get, Router};
use iris_client::IrisClient;
use ris_client::RisClient;

use crate::cache::{self, RedisCache};

pub mod station_board;

#[derive(Clone)]
pub struct CustomEndpointState {
    iris_client: Arc<IrisClient>,
    ris_client: Arc<RisClient>,
    cache: Arc<RedisCache>,
}

pub fn router(redis: Arc<redis::Client>, db_client_id: &str, db_api_key: &str) -> Router {
    let ris_client = Arc::new(RisClient::new(None, None, None, db_client_id, db_api_key));

    let shared_state = Arc::new(CustomEndpointState {
        iris_client: Arc::new(IrisClient::default()),
        ris_client,
        cache: Arc::new(cache::RedisCache::new(redis)),
    });

    Router::new()
        .route("/station_board/:id", get(station_board::station_board))
        .with_state(shared_state)
}
