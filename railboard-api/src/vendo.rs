use std::sync::Arc;

use axum::{routing::get, Router};
use vendo_client::VendoClient;

mod journey_details;
mod location_search;
mod station_board;

pub struct VendoState {
    vendo_client: VendoClient,
}

pub fn router() -> Router {
    let vendo_client = VendoClient::default();

    let shared_state = Arc::new(VendoState { vendo_client });

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
