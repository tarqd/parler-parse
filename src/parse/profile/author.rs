use super::super::media::SimpleImage;
use super::super::util::ShouldSkip;
use super::prelude::*;
use super::Badge;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    #[html(selector = "span.author--name,span.profile--name", attr = "inner")]
    pub name: Option<String>,
    #[html(
        selector = "span.author--username,span.profile--username",
        attr = "inner"
    )]
    pub username: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[html(selector = "div.ch--avatar--badge--wrapper img", attr = "src")]
    pub badge: Option<Badge>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[html(selector = "div.ch--avatar--wrapper ")]
    pub avatar: Option<SimpleImage>,
}
