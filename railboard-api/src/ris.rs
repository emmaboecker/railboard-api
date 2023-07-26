use std::sync::Arc;

use axum::{routing::get, Router};
use reqwest::Client;
// use reqwest::Certificate;
// use reqwest::Proxy;
use ris_client::RisClient;

use crate::cache::RedisCache;

pub mod journey_details;
pub mod journey_search;
pub mod station_board;
pub mod station_information;
pub mod station_search_by_name;

pub struct RisState {
    ris_client: Arc<RisClient>,
    cache: Arc<RedisCache>,
}

pub fn router(redis: Arc<redis::Client>, db_client_id: &str, db_api_key: &str) -> Router {
    let client = Client::builder()
        // .add_root_certificate(Certificate::from_pem(include_bytes!("../../mitm.pem")).unwrap())
        // .proxy(Proxy::all("http://localhost:8080").unwrap())
        .build()
        .unwrap();

    let ris_client = Arc::new(RisClient::new(
        Some(client),
        None,
        None,
        db_client_id,
        db_api_key,
    ));

    let shared_state = Arc::new(RisState {
        ris_client,
        cache: Arc::new(RedisCache::new(redis)),
    });

    Router::new()
        .route(
            "/journey_search/:category/:number",
            get(journey_search::journey_search),
        )
        .route(
            "/journey_details/:id",
            get(journey_details::journey_details),
        )
        .route("/station_board/:eva", get(station_board::station_board))
        .route(
            "/station/:eva",
            get(station_information::station_information),
        )
        .route(
            "/station_search/:query",
            get(station_search_by_name::station_search_by_name),
        )
        .with_state(shared_state)
}
