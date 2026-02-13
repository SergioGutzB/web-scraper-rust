use crate::error::Result;
use reqwest::blocking::Client;
use std::time::Duration;

pub struct HttpFetcher {
    client: reqwest::blocking::Client,
}

impl HttpFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("RustWebScraper/1.0")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    pub fn fetch(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send()?;
        let response = response.error_for_status()?;
        let html = response.text()?;
        Ok(html)
    }
}
