use crate::fetcher::HttpFetcher;
use crate::models::PageData;
use crate::parser::HtmlParser;
use std::sync::{Arc, mpsc};
use threadpool::ThreadPool;

pub struct ScraperEngine {
    pub fetcher: Arc<HttpFetcher>,
    pub num_threads: usize,
}

impl ScraperEngine {
    pub fn new(num_threads: usize) -> Self {
        let fetcher = Arc::new(HttpFetcher::new());

        Self {
            fetcher,
            num_threads,
        }
    }

    pub fn scrape_urls(&self, urls: Vec<String>) -> Vec<PageData> {
        let (tx, rx) = mpsc::channel();
        let pool = ThreadPool::new(self.num_threads);

        for url in urls {
            let tx_clone = tx.clone();
            let fetcher = self.fetcher.clone();

            pool.execute(move || match Self::scrape_single(&fetcher, &url) {
                Ok(page_data) => tx_clone.send(page_data).unwrap(),
                Err(err) => eprintln!("Error scraping {}: {}", url, err),
            });
        }

        drop(tx);
        pool.join();
        rx.iter().collect()
    }

    pub fn scrape_single(fetcher: &HttpFetcher, url: &str) -> crate::error::Result<PageData> {
        let html = fetcher.fetch(url)?;
        HtmlParser::parse(&html, url.to_string())
    }
}
