use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PageData {
    pub url: String,
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Heading {
    pub level: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub href: String,
    pub text: Option<String>,
}

impl PageData {
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            meta_description: None,
            headings: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn error(url: String) -> Self {
        Self {
            url,
            title: None,
            meta_description: None,
            headings: Vec::new(),
            links: Vec::new(),
        }
    }
}
