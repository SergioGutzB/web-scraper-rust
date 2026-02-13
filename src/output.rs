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
