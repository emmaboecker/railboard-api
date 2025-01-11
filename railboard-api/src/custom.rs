use std::sync::Arc;

use axum::{routing::get, Router};

use crate::SharedState;

pub mod station_board;
pub mod station_board_v2;

pub fn router_v1() -> Router<Arc<SharedState>> {
    Router::new().route("/station_board/:id", get(station_board::station_board))
}

pub fn router_v2() -> Router<Arc<SharedState>> {
    Router::new().route(
        "/station_board/:id",
        get(station_board_v2::station_board_v2),
    )
}
