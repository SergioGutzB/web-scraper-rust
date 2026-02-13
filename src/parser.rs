use crate::error::Result;
use crate::models::{Heading, Link, PageData};
use scraper::{Html, Selector};

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse(html: &str, url: String) -> Result<PageData> {
        let document = Html::parse_document(html);

        let title = Self::extract_title(&document);
        let meta_description = Self::extract_meta_description(&document);
        let headings = Self::extract_headings(&document);
        let links = Self::extract_links(&document);

        Ok(PageData {
            url,
            title,
            meta_description,
            headings,
            links,
        })
    }

    fn extract_title(document: &Html) -> Option<String> {
        let selector = Selector::parse("title").unwrap();

        document
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_string())
    }

    fn extract_meta_description(document: &Html) -> Option<String> {
        let selector = Selector::parse("meta[name=\"description\"]").unwrap();

        document
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .map(|s| s.to_string())
    }

    fn extract_headings(document: &Html) -> Vec<Heading> {
        let selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();

        document
            .select(&selector)
            .map(|element| Heading {
                level: element.value().name().to_string(),
                text: element.text().collect::<String>().trim().to_string(),
            })
            .collect()
    }

    fn extract_links(document: &Html) -> Vec<Link> {
        let selector = Selector::parse("a").unwrap();

        document
            .select(&selector)
            .filter_map(|element| {
                element.value().attr("href").map(|href| {
                    let text_content = element.text().collect::<String>().trim().to_string();
                    Link {
                        href: href.to_string(),
                        text: if text_content.is_empty() {
                            None
                        } else {
                            Some(text_content)
                        },
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_html() -> &'static str {
        r#"
        <html>
            <head>
                <title>  Rust Programming  </title>
                <meta name="description" content="Learn Rust programming language">
            </head>
            <body>
                <h1>Welcome to Rust</h1>
                <h2>Getting Started</h2>
                <h3>Installation</h3>
                <a href="https://rust-lang.org">Official Site</a>
                <a href="/docs">Documentation</a>
                <a>Link without href</a>
            </body>
        </html>
        "#
    }

    #[test]
    fn test_parse_returns_correct_url() {
        let html = sample_html();
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(result.url, "https://test.com");
    }

    #[test]
    fn test_extract_title() {
        let html = sample_html();
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(result.title, Some("Rust Programming".to_string()));
    }

    #[test]
    fn test_extract_title_missing() {
        let html = r#"<html><head></head></html>"#;
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(result.title, None);
    }

    #[test]
    fn test_extract_meta_description() {
        let html = sample_html();
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(
            result.meta_description,
            Some("Learn Rust programming language".to_string())
        );
    }

    #[test]
    fn test_extract_headings() {
        let html = sample_html();
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(result.headings.len(), 3);
        assert_eq!(result.headings[0].level, "h1");
        assert_eq!(result.headings[0].text, "Welcome to Rust");
        assert_eq!(result.headings[1].level, "h2");
        assert_eq!(result.headings[1].text, "Getting Started");
        assert_eq!(result.headings[2].level, "h3");
        assert_eq!(result.headings[2].text, "Installation");
    }

    #[test]
    fn test_extract_links_ignores_missing_href() {
        let html = sample_html();
        let result = HtmlParser::parse(html, "https://test.com".to_string()).unwrap();
        assert_eq!(result.links.len(), 2);
        assert_eq!(result.links[0].href, "https://rust-lang.org");
        assert_eq!(result.links[0].text, Some("Official Site".to_string()));
        assert_eq!(result.links[1].href, "/docs");
        assert_eq!(result.links[1].text, Some("Documentation".to_string()));
    }
}
