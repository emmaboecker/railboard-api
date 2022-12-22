use axum::{routing::get, Router};

mod station_board;

pub fn router() -> Router {
    Router::new().route("/station_board/:id", get(station_board::station_board))
}
