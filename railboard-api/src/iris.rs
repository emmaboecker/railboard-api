use std::sync::Arc;

use axum::{routing::get, Router};
use iris_client::IrisClient;

use crate::cache::{self, RedisCache};

pub mod station_board;

#[derive(Clone)]
pub struct IrisState {
    iris_client: Arc<IrisClient>,
    cache: Arc<RedisCache>,
}

pub fn router(redis: Arc<redis::Client>) -> Router {
    let iris_client = Arc::new(IrisClient::default());

    let shared_state = Arc::new(IrisState {
        iris_client,
        cache: Arc::new(cache::RedisCache::new(redis)),
    });

    Router::new()
        .route("/station_board/:id", get(station_board::station_board))
        .with_state(shared_state)
}
