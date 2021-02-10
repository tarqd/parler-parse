use super::{super::media::SimpleImage, card::*, comment::*, prelude::*, timestamp::RelTimestamp};

use std::str::FromStr;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPost {
    echo_by: Option<EchoBy>,
    #[html(
        selector = "div.card--post-container span.post,div.card--post-container span.echo--parent, div.card--post-container span.echo--root"
    )]
    cards: Vec<PostCard>,
    #[html(selector = "div.comments-list--container div.comment--card--wrapper")]
    comments: Vec<Comment>,
    // we get grab the post id from the comments
    #[html(selector = "div.comments-list--container", attr = "id")]
    post_id: Option<IDFromSuffix>,
    #[html(selector = "div.card--body > p a.at", attr = "inner")]
    mentions: Option<Vec<String>>,
    #[html(selector = "div.card--footer div.post--actions")]
    engagements: Option<PostCounts>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCounts {
    #[html(
        selector = ".pa--item--wrapper:nth-child(1) span.pa--item--count",
        attr = "inner"
    )]
    comment_count: Option<i64>,
    #[html(
        selector = ".pa--item--wrapper:nth-child(2) span.pa--item--count",
        attr = "inner"
    )]
    echo_count: Option<i64>,
    #[html(
        selector = ".pa--item--wrapper:nth-child(3) span.pa--item--count",
        attr = "inner"
    )]
    upvote_count: Option<i64>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "div.card--post-container > div.echo-byline--wrapper")]
struct EchoBy {
    #[html(selector = "div.eb--statement", attr = "inner")]
    name: EchoByAuthor,
    #[html(selector = "div.eb--profile-pic")]
    avatar: Option<SimpleImage>,
    #[html(selector = "div.eb--timestamp span.reblock")]
    #[serde(flatten)]
    rel_ts: Option<RelTimestamp>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromText)]
struct EchoByAuthor(String);

impl FromStr for EchoByAuthor {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut it = s.trim().split_whitespace();
        if let (Some("Echoed"), Some("By")) = (it.next(), it.next()) {
            Ok(Self(it.collect::<Vec<&str>>().join(" ")))
        } else {
            Err(Self::Err::SourceNotFound)
        }
    }
}
