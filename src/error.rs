use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("HTML parse error: {0}")]
    HtmlParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
}

pub type Result<T> = std::result::Result<T, ScraperError>;
