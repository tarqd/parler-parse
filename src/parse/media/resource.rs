use super::prelude::*;
use std::str::FromStr;
use super::super::util::{Identifier, ShouldSkip};
use super::simple::UrlParts;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLink {
    label: Option<String>,
    url_raw: String,
    #[serde(flatten)]
    location: Option<UrlParts>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[serde(flatten)]
    id: Option<Identifier>,
    
}

impl FromHtml for ResourceLink {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let sel = Selector::parse(
            "
        a[href],
        img[src],
        video[src],
        video source[src],
        iframe[src],
        embed[src],
        audio[src],
        audio[src]",
        )
        .unwrap();
        let mut current = select.select_elements(&sel);
        let first = current.next().ok_or(())?;

        let location = first
            .value()
            .attr("href")
            .or_else(|| first.value().attr("src"))
            .ok_or_else(|| unhtml::Error::SourceNotFound)?
            ;
        let id = IDFromUrl::from_str(location)
            .ok()
            .map(|v| v.into());


        Ok(ResourceLink {
            url_raw: location.to_string(),
            location: UrlParts::from_str(location).ok(),
            id,
            label: (first
                .value()
                .attr("alt")
                .or_else(|| first.value().attr("title")))
            .map(String::from)
            .or_else(|| Some(first.text().map(str::trim).collect::<Vec<&str>>().concat())),
        })
    }
}
