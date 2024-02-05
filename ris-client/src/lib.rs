use async_lock::Semaphore;

mod helpers;
mod error;
pub use error::*;

mod endpoints;
mod request;

pub use endpoints::*;

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
}
