use super::super::util::ShouldSkip;
use super::{prelude::*, ResourceLink};
use super::{MediaKind, MediaMetadata};

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaItem {
    kind: Option<MediaKind>,
    #[serde(flatten)]
    meta: MediaMetadata,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[html(selector = "div.mc-website--image,
    div.mc-article--image,
    div.mc-video--image,
    div.mc-audio--image
    ")]
    image: Option<super::SimpleImage>,
    #[html(selector = "div.mc-video--wrapper,
        div.mc-image--wrapper,
        div.mc-article--wrapper,
        div.mc-iframe-embed--wrapper,
        div.mc-audio--wrapper,
        div.mc-website--wrapper")]
    source: Option<ResourceLink>,
    #[html(selector = "div.mc-image--modal", attr = "id")]
    numeric_id: Option<IDFromSuffix>,
}
