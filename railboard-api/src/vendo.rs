use axum::{routing::get, Router};

mod journey_details;
mod location_search;
mod station_board;

pub fn router() -> Router {
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
