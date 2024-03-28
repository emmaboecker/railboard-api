use std::sync::Arc;

use axum::{Router, routing::get};

use crate::SharedState;

pub mod station_board;

pub fn router() -> Router<Arc<SharedState>> {
    Router::new()
        .route("v1/station_board/:eva", get(station_board::zugportal_station_board))
}
