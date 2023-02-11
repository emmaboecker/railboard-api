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
pub mod ris; 
pub mod vendo;
pub mod custom; 

mod helpers; 

pub use helpers::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        vendo::station_board::station_board,
        vendo::location_search::location_search, 
        vendo::journey_details::journey_details,
        iris::station_board::station_board,
        ris::journey_search::journey_search,
        ris::journey_details::journey_details,
        ris::station_board::station_board,
        custom::station_board::station_board,
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
        iris_client::station_board::ReplacedTrain,
        iris_client::station_board::message::Message,
        iris_client::station_board::message::MessageStatus,
        iris_client::station_board::message::MessagePriority,
        // Ris stuff
        ris_client::RisError,
        ris_client::RisUnauthorizedError,
        ris_client::ZugportalError,
        ris_client::journey_search::RisJourneySearchElement,
        ris_client::journey_search::RisJourneySearchSchedule,
        ris_client::journey_search::RisJourneySearchTransport,
        ris::journey_details::RisJourneyDetails,
        ris::journey_details::RisJourneyStop,
        ris_client::journey_details::JourneyDetailsMessage,
        ris_client::journey_details::Transport,
        ris::journey_details::JourneyStopTime,
        ris::journey_details::JourneyStopAdministration,
        ris::station_board::RisStationBoard,
        ris::station_board::RisStationBoardItem,
        ris::station_board::RisStationBoardItemAdministration,
        ris::station_board::DepartureArrival,
        // Custom stuff
        custom::station_board::StationBoard,
        custom::station_board::StationBoardItem,
        custom::station_board::StationBoardItemAdministration,
        custom::station_board::DepartureArrival,
        custom::station_board::IrisInformation,
        
    )),
    tags(
        (name = "Iris", description = "API using the Iris API as Backend"),
        (name = "Ris", description = "API using the Ris API as Backend"),
        (name = "Custom", description = "API not using a single API as Backend, but rather a combination of multiple sources"),
        (name = "Vendo", description = "API using the Vendo API as Backend"),
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

    let ris_api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY env variable is not set");
    let ris_client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID env variable is not set");

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .nest("/vendo/v1", vendo::router(redis_client.clone()))
        .nest("/iris/v1", iris::router(redis_client.clone()))
        .nest("/ris/v1", ris::router(redis_client.clone(), &ris_client_id, &ris_api_key))
        .nest("/v1", custom::router(redis_client.clone(), &ris_client_id, &ris_api_key))
        .fallback(|| async { "Nothing here :/" });
    
    let bind_addr = std::env::var("API_URL").unwrap_or_else(|_| String::from("0.0.0.0:8080"));

    let server = Server::bind(&bind_addr.parse().unwrap()).serve(app.into_make_service());
    tracing::info!("Listening on {}", bind_addr);
    server.await.unwrap();
}
