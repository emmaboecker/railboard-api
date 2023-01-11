use axum::{Router, Server};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod error;
pub mod types;
pub mod vendo;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let app = Router::new()
        .nest("/vendo/v1", vendo::router())
        .fallback(|| async { "Nothing here :/" });
    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());
    tracing::info!("Listening on http://localhost:8080/");
    server.await.unwrap();
}
