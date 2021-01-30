use super::derive::*;
use super::opengraph::OGMeta;
use super::parser::ElemIter;
use super::parser::FromHtml;
use super::post::ParlerPost;
use unhtml::scraper::ElementRef;
use unhtml::Result;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta: OGMeta,
    pub post: ParlerPost,
}
