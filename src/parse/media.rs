use super::derive::*;
use super::parser::*;
use std::str::FromStr;
use unhtml::Text;
use url::ParseError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MediaKind {
    Video,
    Audio,
    Image,
    Article,
    Basic,
    IframeEmbed,
    Website,
}
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "img")]
pub struct SimpleImage {
    #[html(attr = "src")]
    pub location: String,
    #[html(attr = "src")]
    pub id: Option<IDFromUrl>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "a")]
pub struct SimpleLink {
    #[html(attr = "href")]
    location: String,
    #[html(attr = "inner")]
    label: Option<String>,
    #[html(attr = "src")]
    id: Option<IDFromUrl>,
}
macro_rules! match_class {
    ($e:expr,  {  $pf:expr => $tf:expr, $($p:expr => $t:expr),*  }) => {
        {
            if $e.has_class($pf, CaseSensitivity::AsciiCaseInsensitive) {
                Ok($tf)
            } $(
                else if $e.has_class($p, CaseSensitivity::AsciiCaseInsensitive) {
                    Ok($t)
                }
            )+
            else {
                Err(().into())
            }
        }
    }
}

impl FromHtml for MediaKind {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let elem = select.next().ok_or(())?;
        let elem = elem.value();

        match_class!(elem, {
            "mc-video--container" => MediaKind::Video,
            "mc-image--container" => MediaKind::Image,
            "mc-basic--container" => MediaKind::Basic,
            "mc-article--container" => MediaKind::Article,
            "mc-website--container" => MediaKind::Website,
            "mc-iframe-embed--container" => MediaKind::IframeEmbed,
            "mc-audio--container" => MediaKind::Audio
        })
    }
}

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

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaItem {
    kind: MediaKind,
    #[serde(flatten)]
    meta: MediaMetadata,
    #[html(selector = "div.mc-image--modal", attr = "id")]
    numeric_id: Option<IDFromSuffix>,
}

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

    #[serde(flatten)]
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
pub struct ResourceLink {
    //    kind: ResourceLinkKind,
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
pub struct IDFromSuffix(String);

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
pub struct IDFromUrl(String);
impl From<IDFromUrl> for String {
    fn from(v: IDFromUrl) -> Self {
        v.0
    }
}
impl FromStr for IDFromUrl {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base = url::Url::parse("https://parler.com/").unwrap();
        let options = url::Url::options().base_url(Some(&base));

        let url = options.parse(s).or_else(|e| Err(()))?;
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
    use html5ever::tree_builder::TreeSink;
    use rayon::collections::linked_list;
    use unhtml::Element;

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
    #[test]
    fn test_video_parse() {
        let str = r#"
        <div class="media-container--wrapper">
        <div class="mc-video--container w--100 p--flex pf--col pf--ac">
        <div class="mc-video--wrapper">
            <video controls>
                <source src="https://video.parler.com/78/6K/test.mp4" type="video/mp4">
                Your browser does not support the video tag.
            </video>
        </div>
        <div class="mc-video--meta--wrapper w--100 p--flex pf--col pf--jsb">
            <span class="mc-video--title reblock"></span>
            <span class="mc-video--excerpt reblock"></span>
            <span class="mc-video--link">
              <a href="https://video.parler.com/78/6K/test_small.mp4" class="p--flex pf--row pf--ac">
                <span class="mc-video--link--icon">
                  <img src="/512ae92f/images/icons/link.svg" alt="">
                </span>
                https://video.parler.com/78/6K/test_small.mp4
              </a>
            </span>
        </div>
    </div></div>
        "#;
        let sel = Selector::parse("div.mc-video--container").unwrap();
        let html = Html::parse_fragment(str);
        let mn : MediaItem = html.select(&sel).element().unwrap();
        assert_eq!(
            mn,
            MediaItem {
                kind: MediaKind::Video,
                numeric_id: None,
                meta: MediaMetadata {
                    title: Some("".into()),
                    excerpt: Some("".into()),
                    link: Some(ResourceLink {
                        label: Some("https://video.parler.com/78/6K/test_small.mp4".into()),
                        location: Some("https://video.parler.com/78/6K/test_small.mp4".into()),
                        id: Some("test".into())
                    })
                }
            }
        )
    }
}
