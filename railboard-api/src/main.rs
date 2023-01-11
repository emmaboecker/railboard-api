use axum::{Router, Server};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[cfg(feature = "cache")]
pub mod cache;
pub mod error;
pub mod types;
pub mod vendo;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    #[cfg(feature = "cache")]
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

    let app = Router::new()
        .nest(
            "/vendo/v1",
            vendo::router(
                #[cfg(feature = "cache")]
                redis_client,
            ),
        )
        .fallback(|| async { "Nothing here :/" });
    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());
    tracing::info!("Listening on http://localhost:8080/");
    server.await.unwrap();
}
