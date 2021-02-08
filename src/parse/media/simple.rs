use super::prelude::*;
use super::super::util::{ UntrimmedString, ShouldSkip} ;
use url::Url;
use std::{result::Result, str::FromStr};
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "img")]
pub struct SimpleImage {
    #[html(attr = "src")]
    url_raw: String,
    #[serde(flatten)]
    #[html(attr = "src")]
    pub location: UrlParts,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[serde(flatten)]
    #[html(attr = "src")]
    pub id: Option<IDFromUrl>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "a")]
pub struct Link {
    #[html(attr = "href")]
    url_raw: String,
    #[serde(flatten)]
    #[html(attr = "href")]
    location: Option<UrlParts>,
    #[html(attr = "inner")]
    label: Option<UntrimmedString>,
    #[html(attr = "href")]
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[serde(flatten)]
    id: Option<IDFromUrl>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromText)]
pub struct UrlParts {
    url: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    host: Option<String>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    is_external: Option<bool>
}

impl FromStr for UrlParts {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base = Url::parse("https://parler.com/").ok();
        let options = Url::options().base_url(base.as_ref());
        
        let url = options.parse(s)
            .map_err(|e| 
                unhtml::Error::TextParseError {
                    text: s.into(),
                    type_name: "link".into(),
                    err: e.to_string(),
                    
                }
            )?;

        Ok(UrlParts {
            url: url.to_string(),
            host: url.host_str().map(String::from),
            is_external: match url.domain() {
                Some(domain) => {
                    let mut it = domain.rsplit('.');
                    Some((Some("com"), Some("parler")) != (it.next(), it.next()))
                },
                _ => None
                
            }
        })
        
    }
}