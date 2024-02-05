use std::sync::Arc;

use axum::{Router, Server};
use dotenvy::dotenv;
use reqwest::{Certificate, Client, Proxy};
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use iris_client::IrisClient;
use ris_client::RisClient;
use vendo_client::VendoClient;

use crate::cache::RedisCache;

pub mod cache;
pub mod error;

pub mod iris;
pub mod ris;
pub mod vendo;
pub mod custom;

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
ris::station_information::station_information,
ris::station_search_by_name::station_search_by_name,
custom::station_board::station_board,
),
components(schemas(
error::RailboardApiError,
error::ErrorDomain,
error::UnderlyingApiError,
// Vendo stuff
vendo_client::VendoError,
vendo_client::shared::Time,
vendo_client::shared::Notice,
vendo_client::shared::HimNotice,
vendo_client::shared::Attribute,
vendo_client::station_board::VendoStationBoard,
vendo_client::station_board::StationBoardElement,
vendo_client::station_board::StationBoardArrival,
vendo_client::station_board::StationBoardDeparture,
vendo_client::location_search::VendoLocationSearchResult,
vendo_client::location_search::VendoLocationSearchCoordinates,
vendo_client::journey_details::VendoJourneyDetails,
vendo_client::journey_details::VendoTrainSchedule,
vendo_client::journey_details::VendoStop,
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
ris_client::journey_details::RisJourneyDetails,
ris_client::journey_details::RisJourneyStop,
ris_client::journey_details::RisJourneyStopEvent,
ris_client::journey_details::RisJourneyStopAdministration,
ris_client::journey_details::RisJourneyStopDisruption,
ris_client::journey_details::RisTransport,
ris_client::journey_details::RisReplacementTransport,
ris_client::journey_details::RisJourneyDetailsMessage,
ris_client::station_board::RisStationBoard,
ris_client::station_board::RisStationBoardItem,
ris_client::station_board::RisStationBoardItemAdministration,
ris_client::station_board::DepartureArrival,
ris_client::station_information::RisStationInformation,
ris_client::station_information::RisStationNameContent,
ris_client::station_information::RisPosition,
ris_client::station_search::RisStationSearchResponse,
ris_client::station_search::RisStationSearchElement,
ris_client::station_search::RisStationSearchTranslatable,
ris_client::station_search::RisStationSearchNameContent,
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
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
            tracing::warn!("REDIS_URL env variable is not set. Using default \"redis://127.0.0.1/\"");
            String::from("redis://127.0.0.1/")
        });
        redis::Client::open(redis_url).expect("Failed create redis client, check redis url")
    };

    let redis_client = Arc::new(redis_client);

    let ris_api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY env variable is not set");
    let ris_client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID env variable is not set");

    let http_client =
        Client::builder()
            // .add_root_certificate(Certificate::from_pem(include_bytes!("../../mitm.pem")).unwrap())
            // .proxy(Proxy::all("http://localhost:8080").unwrap())
            .build()
            .unwrap();

    let ris_client = Arc::new(RisClient::new(
        Some(http_client.clone()),
        None,
        None,
        &ris_client_id,
        &ris_api_key,
    ));

    let iris_client = Arc::new(IrisClient::new(Some(http_client.clone()), None, None));

    let vendo_client = Arc::new(VendoClient::new(Some(http_client.clone()), None, None));

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .nest("/vendo/v1", vendo::router())
        .nest("/iris/v1", iris::router())
        .nest("/ris/v1", ris::router())
        .nest("/v1", custom::router()).with_state(Arc::new(SharedState {
        vendo_client,
        ris_client,
        iris_client,
        cache: RedisCache::new(redis_client),
    }))
        .fallback(|| async { "Nothing here :/" });

    let bind_addr = std::env::var("API_URL").unwrap_or_else(|_| String::from("0.0.0.0:8080"));

    let server = Server::bind(&bind_addr.parse().unwrap()).serve(app.into_make_service()).with_graceful_shutdown(shutdown_hook());
    tracing::info!("Listening on {}", bind_addr);
    server.await.unwrap();
}

pub struct SharedState {
    vendo_client: Arc<VendoClient>,
    ris_client: Arc<RisClient>,
    iris_client: Arc<IrisClient>,
    cache: RedisCache,
}

async fn shutdown_hook() {
    #[cfg(unix)]
    tokio::select! {
        _ = async {
            let mut signal = ::tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();
            signal.recv().await;
        } => {
            tracing::info!("Received SIGINT. Shutting down.");
        },
        _ = async {
            let mut signal = ::tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
            signal.recv().await;
        } => {
            tracing::info!("Received SIGTERM. Shutting down.");
        },
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await.unwrap()
}
