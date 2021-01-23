use super::opengraph::OGMeta;
use super::post::ParlerPost;
use super::derive::*;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta : OGMeta,
    pub post : ParlerPost,

}