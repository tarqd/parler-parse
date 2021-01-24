use super::derive::*;
use super::parser::*;
use std::str::FromStr;
use unhtml::Text;
use url::ParseError;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "div.media-container--wrapper")]
pub struct MediaContainer {
    #[html(selector = ".sensitive--content--wrapper")]
    is_sensitive_content: ElementExists,
    media_items: Vec<MediaItem>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "
    div.mc-basic--container,
    div.mc-iframe-embed--container,
    div.mc-image--container,
    div.mc-video--container,
    div.mc-website--container")]
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
    #[html(selector = "div.mc-image--modal", attr = "id")]
    numeric_id: Option<IDFromSuffix>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "a")]
pub struct Link {
    #[html(attr = "inner")]
    label: Option<String>,
    #[html(attr = "href")]
    destination: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceLinkKind {
    Anchor,
    IFrame,
    Video,
    Audio,
    Image,
    Embed,
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLink {
    kind: ResourceLinkKind,
    label: Option<String>,
    location: Option<String>,
    id: Option<String>,
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

        let location = first
            .value()
            .attr("href")
            .or_else(|| first.value().attr("src"))
            .map(String::from);
        let id = location
            .as_ref()
            .and_then(|v| IDFromUrl::from_str(v.as_ref()).ok())
            .map(|v| v.into());

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
            location,
            id,
            label: (first
                .value()
                .attr("alt")
                .or_else(|| first.value().attr("title")))
            .map(String::from)
            .or_else(|| Some(first.text().map(str::trim).collect::<Vec<&str>>().concat())),
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ElementExists(bool);

impl FromHtml for ElementExists {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        Ok(Self(select.next().is_some()))
    }
}

#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
struct IDFromSuffix(String);

impl FromStr for IDFromSuffix {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.rsplit('-')
            .next()
            .map(|v| Self(v.into()))
            .ok_or(().into())
    }
}

#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
struct IDFromUrl(String);
impl From<IDFromUrl> for String {
    fn from(v: IDFromUrl) -> Self {
        v.0
    }
}
impl FromStr for IDFromUrl {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = url::Url::parse(s).or_else(|e| Err(()))?;
        let domain = url.domain().ok_or(())?;

        if domain.ends_with(".parler.com") {
            let path = std::path::Path::new(url.path());
            let id = path.file_stem().ok_or(())?.to_str().ok_or(())?;

            return id
                .split('_')
                .next()
                .map(|s| Self(s.into()))
                .ok_or(().into());
        }
        Err(().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_suffix_parser_image_modal_num_id() {
        let str = r#"<div id="mc-image--modal--12345" class="mc-image--modal"></div>"#;
        let doc = Html::parse_fragment(str);
        let sel = Selector::parse(&"div").unwrap();
        let id = IDFromSuffix::from_attr(&mut doc.select(&sel), "id").unwrap();
        assert_eq!("12345", id.0)
    }

    #[test]
    fn img_path_to_id() {
        let str = r#"<img src="https://api.parler.com/l/test" alt="Image">"#;
        let doc = Html::parse_fragment(str);
        let sel = Selector::parse(&"img").unwrap();
        let id = IDFromUrl::from_attr(&mut doc.select(&sel), "src").unwrap();
        assert_eq!("test", id.0)
    }
    #[test]
    fn video_path_to_id() {
        let str = r#"
            <video src="https://video.parler.com/4h/tH/test_small.mp4" type="video/mp4">
        "#;
        let doc = Html::parse_fragment(str);
        let sel = Selector::parse(&"video").unwrap();
        let id = IDFromUrl::from_attr(&mut doc.select(&sel), "src").unwrap();
        assert_eq!("test", id.0)
    }
}
