use super::prelude::*;
use super::super::{
    profile::Author,
    media::*
};

#[derive(Debug, PartialEq, FromHtml, Serialize, Deserialize)]
#[html(selector = "div#hero--wrapper")]
pub struct ParlerProfile {
    #[serde(flatten)]
    #[html(selector = "#hero")]
    user: Author,
    #[html(selector = "#hero--top")]
    banner: Option<SimpleImage>,
    #[html(selector = "#hero--bottom div.profile-photo-image")]
    avatar: Option<SimpleImage>,
    #[html(selector = "#hero--bottom span.profile--bio", attr = "inner")]
    bio: Option<String>,
    #[html(selector = "#hero--bottom span.profile--bio a")]
    bio_links: Vec<Link>,
}
