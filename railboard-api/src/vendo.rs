use std::sync::Arc;

use axum::{Router, routing::get};

use crate::SharedState;

pub mod journey_details;
pub mod location_search;
pub mod station_board;

pub fn router() -> Router<Arc<SharedState>> {
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
}
