use super::prelude::*;
use super::{ResourceLink};


#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector="div.mc-article--meta--wrapper,
div.mc-basic--meta--wrapper,
div.mc-iframe-embed--meta--wrapper,
div.mc-video--meta--wrapper,
div.mc-article--meta--wrapper,
div.mc-website--meta--wrapper,
div.mc-image--meta--wrapper,
div.mc-image--modal")]
pub struct MediaMetadata {
    #[html(
        selector = "span.mc-article--title,
                span.mc-basic--title,
                span.mc-iframe-embed--title,
                span.mc-video--title,
                span.mc-website--title",
        attr = "inner"
    )]
    title: Option<String>,

    #[html(selector = "span.mc-article--link,
                       span.mc-basic--link,
                       span.mc-iframe-embed--link,
                       span.mc-video--link,
                       span.mc-website--link,
                       div.mc-image--modal--element--wrapper
    ")]
    link: Option<ResourceLink>,

    #[html(
        selector = "span.mc-article--excerpt,
                span.mc-basic--excerpt,
                span.mc-iframe-embed--excerpt,
                span.mc-video--excerpt,
                span.mc-website--excerpt",
        attr = "inner"
    )]
    excerpt: Option<String>,
}