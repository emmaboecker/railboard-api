use std::sync::Arc;

use axum::{routing::get, Router};
use vendo_client::VendoClient;

use crate::cache::RedisCache;
#[cfg(feature = "cache")]
use crate::cache::{self};

pub mod journey_details;
pub mod location_search;
pub mod station_board;

pub struct VendoState {
    vendo_client: VendoClient,
    cache: RedisCache,
}

pub fn router(redis: redis::Client) -> Router {
    let vendo_client = VendoClient::default();

    let shared_state = Arc::new(VendoState {
        vendo_client,
        cache: cache::RedisCache::new(redis),
    });

    Router::new()
        .route("/station_board/:id", get(station_board::station_board))
        .route(
            "/journey_details/:id",
            get(journey_details::journey_details),
        )
        .route(
            "/location_search/:query",
            get(location_search::location_search),
        )
        .with_state(shared_state)
}
