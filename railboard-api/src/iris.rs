use std::sync::Arc;

use axum::{routing::get, Router};

use crate::SharedState;

pub mod station_board;

pub fn router() -> Router<Arc<SharedState>> {
    Router::new().route("/station_board/:id", get(station_board::station_board))
}
