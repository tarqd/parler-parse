use super::super::media::*;
use super::super::profile::Author;
use super::prelude::*;
use super::timestamp::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PostCardType {
    Post,
    EchoParent,
    EchoRoot,
}

impl FromHtml for PostCardType {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let elem = select.next().ok_or(())?.value();
        if elem.has_class("echo--parent", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(PostCardType::EchoParent)
        } else if elem.has_class("echo--root", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(PostCardType::EchoRoot)
        } else if elem.has_class("post", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(PostCardType::Post)
        } else {
            unhtml::Result::Err(unhtml::Error::TextParseError {
                text: elem.attr("class").unwrap_or_default().to_string(),
                type_name: "PostCardType".to_string(),
                err: "must have one of these classes: echo--root,echo--parent,post".to_string(),
            })
        }
    }
}
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCard {
    kind: PostCardType,
    #[html(
        selector = "div.card--header  a.card-meta--row, div.card--header div.ch--avatar-col,  div.ch--avatar--wrapper"
    )]
    author: Option<Author>,
    #[serde(flatten)]
    #[html(selector = "div.card--header span.card-meta--row span.post--timestamp")]
    rel_timestamp: RelTimestamp,
    #[html(selector = "div.card--body > p", attr = "inner")]
    body: Option<UntrimmedString>,
    #[html(
        selector = "span.card-meta--row span.impressions--wrapper span.impressions--count",
        attr = "inner"
    )]
    impression_count: Option<i64>,
    #[html(selector = ":scope > div.card--body
       ")]
    #[serde(flatten)]
    media_container: Option<MediaContainer>,
}
