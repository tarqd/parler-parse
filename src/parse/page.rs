use super::{derive::*, media::SimpleImage, media::SimpleLink};
use super::opengraph::OGMeta;
use super::parser::ElemIter;
use super::parser::FromHtml;
use super::post::ParlerPost;
use super::parser::Selector;
use unhtml::{Select, scraper::{self, ElementRef, selector::Simple}};
use unhtml::Result;
use super::post::Author;
use super::media::{ResourceLink};
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta: OGMeta,
    pub posts: Vec<ParlerPost>,
    pub profile: Option<ParlerProfile>
}



#[derive(Debug, PartialEq, FromHtml, Serialize,Deserialize)]
#[html(selector="div#hero--wrapper")]
pub struct ParlerProfile {
    #[html(selector="#hero")]
    user: Option<Author>,
    #[html(selector="#hero--top")]
    banner: Option<SimpleImage>,
    #[html(selector="#hero--bottom div.profile-photo-image")]
    profile_image: Option<SimpleImage>,
    #[html(selector="#hero--bottom span.profile--bio", attr="inner")]
    bio: Option<String>,
    #[html(selector="#hero--bottom span.profile--bio a")]
    bio_links: Vec<SimpleLink>
}




