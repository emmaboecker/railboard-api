use async_lock::Semaphore;

mod error;
pub use error::*;

mod endpoints;
pub use endpoints::*;

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
            client: client.unwrap_or_else(reqwest::Client::new),
            base_url: base_url.unwrap_or_else(|| String::from("https://iris.noncd.db.de")),
            semaphore: Semaphore::new(concurrent_requests.unwrap_or(100)),
        }
    }
}
