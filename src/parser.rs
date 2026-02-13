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
