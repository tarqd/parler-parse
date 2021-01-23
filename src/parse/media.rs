use super::parser::*;
use super::derive::*;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector="div.media-container--wrapper")]
pub struct MediaContainer {
    #[html(selector = ".sensitive--content--wrapper", attr = "id")]
    sensitive_id: Option<String>,
    media_items: Vec<MediaItem>,
}

#[derive(FromHtml, Debug, PartialEq,Serialize, Deserialize)]
#[html(
selector = "
    div.mc-basic--container,
    div.mc-iframe-embed--container,
    div.mc-image--container,
    div.mc-video--container,
    div.mc-website--container"

)]
pub struct MediaItem {
    #[html(selector = "div.mc-article--meta--wrapper,
     div.mc-basic--meta--wrapper,
     div.mc-iframe-embed--meta--wrapper,
     div.mc-video--meta--wrapper,
     div.mc-article--meta--wrapper,
     div.mc-website--meta--wrapper,
     div.mc-image--meta--wrapper,
     div.mc-image--modal")]
    meta: MediaMetadata,
}

#[derive(FromHtml, Debug, PartialEq,Serialize, Deserialize)]
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

#[derive(FromHtml, Debug, PartialEq,Serialize,Deserialize)]
#[html(selector = "a")]
pub struct Link {
    #[html(attr = "inner")]
    label: Option<String>,
    #[html(attr = "href")]
    destination: Option<String>,
}

#[derive(Debug, PartialEq,Serialize, Deserialize)]
pub enum ResourceLinkKind {
    Anchor,
    IFrame,
    Video,
    Audio,
    Image,
    Embed,
    Unknown,
}

#[derive(Debug, PartialEq,Serialize, Deserialize)]
pub struct ResourceLink {
    kind: ResourceLinkKind,
    label: Option<String>,
    location: Option<String>,
}

impl FromHtml for ResourceLink {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let sel = Selector::parse(
            "
        a[href],
        img[src],
        video[src],
        video source[src],
        iframe[src],
        embed[src],
        audio[src],
        audio[src]",
        )
            .unwrap();
        let mut current = select.select_elements(&sel);
        let first = current.next().ok_or(())?;

        Ok(ResourceLink {
            kind: match first.value().name().to_ascii_lowercase().as_str() {
                "a" => ResourceLinkKind::Anchor,
                "img" => ResourceLinkKind::Image,
                "video" => ResourceLinkKind::Video,
                "iframe" => ResourceLinkKind::IFrame,
                "audio" => ResourceLinkKind::Audio,
                "embed" => ResourceLinkKind::Embed,
                "source" => first.parent_element().map_or_else(
                    || ResourceLinkKind::Unknown,
                    |v| match v.value().name() {
                        "video" => ResourceLinkKind::Video,
                        "audio" => ResourceLinkKind::Audio,
                        _ => ResourceLinkKind::Unknown,
                    },
                ),
                _ => ResourceLinkKind::Unknown,
            },
            location: first
                .value()
                .attr("href")
                .or_else(|| first.value().attr("src"))
                .map(String::from),

            label: (first
                .value()
                .attr("alt")
                .or_else(|| first.value().attr("title")))
                .map(String::from)
                .or_else(|| Some(first.text().map(str::trim).collect::<Vec<&str>>().concat())),
        })
    }
}