use super::{
    prelude::*,
    card::*,
    comment::*,
};

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlerPost {
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