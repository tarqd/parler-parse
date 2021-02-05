use super::media::ResourceLink;
use super::opengraph::OGMeta;
use super::parser::ElemIter;
use super::parser::FromHtml;
use super::parser::Selector;
use super::post::Author;
use super::post::ParlerPost;
use super::{derive::*, media::SimpleImage, media::SimpleLink};
use unhtml::Result;
use unhtml::{
    scraper::{self, selector::Simple, ElementRef},
    Select,
};
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta: OGMeta,
    #[html(selector = "main div.post--card--wrapper")]
    pub posts: Vec<ParlerPost>,
    pub profile: Option<ParlerProfile>,
}

#[derive(Debug, PartialEq, FromHtml, Serialize, Deserialize)]
#[html(selector = "div#hero--wrapper")]
pub struct ParlerProfile {
    #[serde(flatten)]
    #[html(selector = "#hero")]
    user: Option<Author>,
    #[html(selector = "#hero--top")]
    banner: Option<SimpleImage>,
    #[html(selector = "#hero--bottom div.profile-photo-image")]
    avatar: Option<SimpleImage>,
    #[html(selector = "#hero--bottom span.profile--bio", attr = "inner")]
    bio: Option<String>,
    #[html(selector = "#hero--bottom span.profile--bio a")]
    bio_links: Vec<SimpleLink>,
}
