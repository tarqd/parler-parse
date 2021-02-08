use super::prelude::*;
use super::MediaItem;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "div.media-container--wrapper")]
pub struct MediaContainer {
    #[html(selector = ".sensitive--content--wrapper")]
    is_sensitive_content: ElementExists,
    #[html(selector="div.mc-video--container,
    div.mc-image--container,
    div.mc-basic--container,
    div.mc-article--container,
    div.mc-website--container,
    div.mc-iframe-embed--container,
    div.mc-audio--container")]
    media_items: Vec<MediaItem>,
}