use super::derive::*;
use super::{
    opengraph::{OGMeta},
    post::ParlerPost,
    profile::ParlerProfile
};

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta: OGMeta,
    #[html(selector = "main div.post--card--wrapper")]
    pub posts: Vec<ParlerPost>,
    pub profile: Option<ParlerProfile>,
}