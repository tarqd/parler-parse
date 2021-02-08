use super::derive::*;
use super::{
    opengraph::{OGMeta},
    post::ParlerPost,
    profile::ParlerProfile,
    util::ShouldSkip,
};

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPage {
    pub opengraph_meta: OGMeta,
    #[html(selector = "main div.post--card--wrapper")]
    pub posts: Vec<ParlerPost>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub profile: Option<ParlerProfile>,
}
