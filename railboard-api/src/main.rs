use std::sync::Arc;

use axum::{Router, Server};
use dotenvy::dotenv;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod cache;
pub mod error;
pub mod types;

pub mod iris;
pub mod vendo;

#[derive(OpenApi)]
#[openapi(
    paths(
        vendo::station_board::station_board,
        vendo::location_search::location_search, 
        vendo::journey_details::journey_details,
        iris::station_board::station_board
    ),
    components(schemas(
        error::RailboardApiError,
        error::ErrorDomain,
        error::UnderlyingApiError,
        // Vendo stuff
        types::Time,
        types::Notice,
        types::HimNotice,
        types::Attribute,
        vendo_client::VendoError,
        vendo::station_board::StationBoard,
        vendo::station_board::StationBoardElement,
        vendo::station_board::StationBoardArrival,
        vendo::station_board::StationBoardDeparture,
        vendo_client::location_search::LocationSearchResult,
        vendo_client::location_search::LocationSearchCoordinates,
        vendo::journey_details::JourneyDetails,
        vendo::journey_details::TrainSchedule,
        vendo::journey_details::Stop,
        // Iris stuff
        iris_client::station_board::IrisStationBoard,
        iris_client::station_board::StationBoardStop,
        iris_client::station_board::StationBoardStopArrival,
        iris_client::station_board::StationBoardStopDeparture,
        iris_client::station_board::RouteStop,
        iris_client::station_board::message::Message,
        iris_client::station_board::message::MessageStatus,
        iris_client::station_board::message::MessagePriority,
    )),
    tags(
        (name = "Vendo", description = "API built on top of the Vendo API"),
        (name = "Iris", description = "API built on top of the Iris API"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let redis_client = {
        let redis_url = match std::env::var("REDIS_URL") {
            Ok(url) => url,
            Err(_) => {
                tracing::warn!("caching is enabled and REDIS_URL env variable is not set. Using default \"redis://127.0.0.1/\"");
                String::from("redis://127.0.0.1/")
            }
        };
        redis::Client::open(redis_url).expect("Failed create redis client, check redis url")
    };

    let redis_client = Arc::new(redis_client);

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .nest("/vendo/v1", vendo::router(redis_client.clone()))
        .nest("/iris/v1", iris::router(redis_client.clone()))
        .fallback(|| async { "Nothing here :/" });
    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());
    tracing::info!("Listening on http://localhost:8080/");
    server.await.unwrap();
}
