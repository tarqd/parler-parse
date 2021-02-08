use super::{
    prelude::*,
    timestamp::*,
    super::{
    profile::{Author},
    media::{MediaContainer}
    }
};

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    #[html(selector = "div.card--comment-container div.card--header div.ch--meta-col, div.card--comment-container div.card--header div.ch--avatar-col")]
    author: Option<Author>,
    #[serde(flatten)]
    #[html(selector = "div.card--comment-container div.card--header span.card-meta--row span.post--timestamp")]
    rel_timestamp: RelTimestamp,

    #[html(selector = "div.card--comment-container div.card--body p", attr = "inner")]
    body: Option<UntrimmedString>,
    #[html(selector = "div.card--comment-container div.card--footer div.comment--actions")]
    engagements: Option<CommentCounts>,

    #[html(selector = ":scope > div.card--comment-container div.card--body")]
    #[serde(flatten)]
    media_container: Option<MediaContainer>,

    // post/026d108991b44cffbb394497aad428e4
    #[html(selector="div.replies-list--container", attr="id")]
    comment_id: Option<IDFromSuffix>,
    #[html(selector="div.replies-list--container > div.reply--card--wrapper")]
    replies: Option<Vec<Comment>>
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentCounts {
    #[html(
        selector = ".ca--item--wrapper:nth-child(1) span.ca--item--count",
        attr = "inner"
    )]
    reply_count: Option<i64>,
    #[html(
        selector = ".ca--item--wrapper:nth-child(2) span.ca--item--count",
        attr = "inner"
    )]
    downvote_count: Option<i64>,
    #[html(
        selector = ".ca--item--wrapper:nth-child(3) span.ca--item--count",
        attr = "inner"
    )]
    upvote_count: Option<i64>,
}