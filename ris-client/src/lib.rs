use async_lock::Semaphore;

mod error;
mod helpers;
pub use error::*;

mod endpoints;
mod request;

pub use endpoints::*;
use reqwest::{Certificate, Client, Proxy};

pub struct RisClient {
    client: reqwest::Client,
    base_url: String,
    semaphore: Semaphore,
    db_client_id: String,
    db_api_key: String,
}

impl RisClient {
    pub fn new(
        client: Option<reqwest::Client>,
        base_url: Option<String>,
        concurrent_requests: Option<usize>,
        db_client_id: &str,
        db_api_key: &str,
    ) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: base_url.unwrap_or_else(|| String::from("https://apis.deutschebahn.com")),
            semaphore: Semaphore::new(concurrent_requests.unwrap_or(100)),
            db_client_id: db_client_id.to_string(),
            db_api_key: db_api_key.to_string(),
        }
    }

    pub fn default_debug(proxy: &str, pem: &[u8], db_client_id: &str, db_api_key: &str) -> Self {
        let http_client = Client::builder()
            .add_root_certificate(Certificate::from_pem(pem).unwrap())
            .proxy(Proxy::all(proxy).unwrap())
            .build()
            .unwrap();
        Self::new(Some(http_client), None, None, db_client_id, db_api_key)
    }
}
