mod error;
mod fetcher;
mod models;
mod output;
mod parser;
mod scraper;

fn main() {
    let urls = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
        "https://github.com".to_string(),
    ];

    let scraper = scraper::ScraperEngine::new(4);
    let data = scraper.scrape_urls(urls);

    println!("Scraped pages: {}", data.len());

    if let Err(e) = output::write_json(&data, "data.json") {
        eprintln!("Error writing JSON: {}", e);
    }

    if let Err(e) = output::write_csv(&data, "data.csv") {
        eprintln!("Error writing CSV: {}", e);
    }

    println!("Done");
}
