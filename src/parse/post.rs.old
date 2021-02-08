use super::derive::*;
use super::media::*;
use super::parser::*;
use super::UntrimmedString;
use std::str::FromStr;
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromText)]
struct ApproxRelTimestampOffset(i64);

impl std::str::FromStr for ApproxRelTimestampOffset {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let num = split.next().ok_or(())?.parse::<i64>().map_err(|_| ())?;
        let unit = split.next().ok_or(())?;
        let mult: i64 = match unit {
            "second" | "seconds" => 1,
            "minute" | "minutes" => 60,
            "hour" | "hours" => 60 * 60,
            "day" | "days" => 60 * 60 * 24,
            "week" | "weeks" => 60 * 60 * 24 * 7,
            // fuzzy
            "month" | "months" => 60 * 60 * 24 * 30,
            "year" | "years" => 60 * 60 * 24 * 30 * 365,
            _ => Err(())?,
        };

        Ok(Self(num * mult * -1))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromHtml)]
struct RelTimestamp {
    #[html(attr = "inner")]
    rel_ts: Option<String>,
    #[html(attr = "inner")]
    approx_ts_offset: Option<ApproxRelTimestampOffset>,
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
    media_container: Option<MediaContainer>,
}

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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum BadgeKind {
    Verified,
    Gold,
    IntegrationPartner,
    Affiliate,
    Private,
    VerifiedComments,
    Parody,
    Employee,
    RealName,
    EarlyAdopter,
}

impl FromStr for BadgeKind {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let options = Url::options();
        let base = Url::parse("https://parler.com/").unwrap();
        let base = options.base_url(Some(&base));
        let url = base.parse(s).map_err(|e| unhtml::Error::TextParseError {
            text: s.into(),
            type_name: "Badge".into(),
            err: e.to_string(),
        })?;
        let parts = url
            .path_segments()
            .and_then(|v| v.last())
            .and_then(|v| v.split(".").next())
            .ok_or_else(|| ())?;

        use BadgeKind::*;

        match parts {
            "00" => Ok(Verified),
            "01" => Ok(Gold),
            "02" => Ok(IntegrationPartner),
            "04" => Ok(Affiliate),
            "05" => Ok(Private),
            "06" => Ok(Parody),
            "07" => Ok(Employee),
            "08" => Ok(RealName),
            "09" => Ok(EarlyAdopter),
            _ => Err(unhtml::Error::from((
                s.to_string(),
                "Badge".to_string(),
                "invalid badge url".to_string(),
            ))),
        }
    }
}

trait ShouldSkip {
    fn should_skip(&self) -> bool;
}

impl<T> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.is_none()
    }
}

// unhtml doesnt support derives from enums and I'm laazy
#[derive(FromText, Debug, Serialize, Deserialize, PartialEq)]
pub struct Badge(BadgeKind);

impl FromStr for Badge {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BadgeKind::from_str(s)?))
    }
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    #[html(selector = "span.author--name,span.profile--name", attr = "inner")]
    pub name: Option<String>,
    #[html(
        selector = "span.author--username,span.profile--username",
        attr = "inner"
    )]
    pub username: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[html(selector = "div.ch--avatar--badge--wrapper img", attr = "src")]
    pub badge: Option<Badge>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[html(selector = "div.ch--avatar--wrapper ")]
    pub avatar: Option<SimpleImage>,
}

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
pub struct Comment {
    #[html(selector = "div.card--header div.ch--meta-col, div.card--header div.ch--avatar-col")]
    author: Option<Author>,
    #[serde(flatten)]
    #[html(selector = "div.card--header span.card-meta--row span.post--timestamp")]
    rel_timestamp: RelTimestamp,

    #[html(selector = "div.card--body p", attr = "inner")]
    body: Option<UntrimmedString>,
    #[html(selector = "div.card--footer div.comment--actions")]
    engagements: Option<CommentCounts>,
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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn badge_parser_card() {
        let html = r#"
        <span class="reblock echo--parent show-echo-arrows">
        <div class="card--header p--flex pf--row">
<div class="ch--col ch--avatar-col">
    <div class="ch--avatar--wrapper">
        <img src="https://images.parler.com/profile_256.jpg" alt="Post Author Profile Pic">
    </div>
    
    <div class="ch--avatar--badge--wrapper">
        
            <img src="/512ae92f/images/badges/00.svg" alt="Badge">
        
    </div>
    
</div>

<div class="ch--col ch--meta-col p--flex pf--col pf--jc">
    
    <a href="https://parler.com/profile/username/posts" class="card-meta--row">
    
        <span class="author--name">name</span>
        <span class="separator">·</span>
        <span class="author--username">@username</span>
    
    </a>
    </div>
    </span>
    </div>
        "#;
        let author = Author::from_html(&html).unwrap();
        assert!(author.badge.is_some());
        assert!(author.avatar.is_some());
        assert_eq!(author.badge, Some(Badge(BadgeKind::Verified)));
    }
}
