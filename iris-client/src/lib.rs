use async_lock::Semaphore;

mod error;
pub use error::*;

mod endpoints;
pub use endpoints::*;
use reqwest::{Certificate, Client, Proxy};

pub mod helpers;

pub struct IrisClient {
    client: reqwest::Client,
    base_url: String,
    semaphore: Semaphore,
}

impl Default for IrisClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: String::from("https://iris.noncd.db.de"),
            semaphore: Semaphore::new(100),
        }
    }
}

impl IrisClient {
    pub fn new(
        client: Option<reqwest::Client>,
        base_url: Option<String>,
        concurrent_requests: Option<usize>,
    ) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: base_url.unwrap_or_else(|| String::from("https://iris.noncd.db.de")),
            semaphore: Semaphore::new(concurrent_requests.unwrap_or(100)),
        }
    }

    pub fn default_debug(proxy: &str, pem: &[u8]) -> Self {
        let http_client = Client::builder()
            .add_root_certificate(Certificate::from_pem(pem).unwrap())
            .proxy(Proxy::all(proxy).unwrap())
            .build()
            .unwrap();

        Self::new(Some(http_client), None, None)
    }
}
