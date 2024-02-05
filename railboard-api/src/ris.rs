use std::sync::Arc;

use axum::{Router, routing::get};

use crate::SharedState;

pub mod journey_details;
pub mod journey_search;
pub mod station_board;
pub mod station_information;
pub mod station_search_by_name;

pub fn router() -> Router<Arc<SharedState>> {
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
}
