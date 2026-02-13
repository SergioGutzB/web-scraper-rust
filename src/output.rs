use crate::error::Result;
use crate::models::PageData;
use std::io::Write;

pub fn write_json(data: &[PageData], filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = std::fs::File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn write_csv(data: &[PageData], filename: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(filename)?;

    writer.write_record(&[
        "url",
        "title",
        "meta_description",
        "headings_count",
        "links_count",
    ])?;

    for page in data {
        writer.write_record(&[
            page.url.as_str(),
            page.title.as_deref().unwrap_or(""),
            page.meta_description.as_deref().unwrap_or(""),
            &page.headings.len().to_string(),
            &page.links.len().to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Heading, Link};

    fn sample_data() -> Vec<PageData> {
        vec![
            PageData {
                url: "https://example.com".to_string(),
                title: Some("Example".to_string()),
                meta_description: Some("An example".to_string()),
                headings: vec![Heading {
                    level: "h1".to_string(),
                    text: "Hello".to_string(),
                }],
                links: vec![Link {
                    href: "https://rust-lang.org".to_string(),
                    text: Some("Rust".to_string()),
                }],
            },
            PageData {
                url: "https://test.com".to_string(),
                title: None,
                meta_description: None,
                headings: vec![],
                links: vec![],
            },
        ]
    }

    #[test]
    fn test_write_json_creates_file() {
        let data = sample_data();
        write_json(&data, "test_1.json").unwrap();
        let content = std::fs::read_to_string("test_1.json").unwrap();

        assert!(content.contains("https://example.com"));
        assert!(std::path::Path::new("test_1.json").exists());

        std::fs::remove_file("test_1.json").unwrap();
    }

    #[test]
    fn test_write_json_content() {
        let data = sample_data();
        write_json(&data, "test_2.json").unwrap();
        let content = std::fs::read_to_string("test_2.json").unwrap();

        assert!(content.contains("Example"));

        std::fs::remove_file("test_2.json").unwrap();
    }

    #[test]
    fn test_write_csv_creates_file() {
        let data = sample_data();
        write_csv(&data, "test_1.csv").unwrap();
        let content = std::fs::read_to_string("test_1.csv").unwrap();

        assert!(content.contains("https://example.com"));
        assert!(std::path::Path::new("test_1.csv").exists());

        std::fs::remove_file("test_1.csv").unwrap();
    }
}
