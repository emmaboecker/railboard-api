pub mod station_board;
use async_lock::Semaphore;

pub struct VendoClient {
    client: reqwest::Client,
    base_url: String,
    semaphore: Semaphore,
}

impl Default for VendoClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: String::from("https://app.vendo.noncd.db.de/"),
            semaphore: Semaphore::new(100),
        }
    }
}

impl VendoClient {
    pub fn new(
        client: Option<reqwest::Client>,
        base_url: Option<String>,
        concurrent_requests: Option<usize>,
    ) -> Self {
        Self {
            client: client.unwrap_or_else(reqwest::Client::new),
            base_url: base_url.unwrap_or_else(|| String::from("https://app.vendo.noncd.db.de/")),
            semaphore: Semaphore::new(concurrent_requests.unwrap_or(100)),
        }
    }
}
