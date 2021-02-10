use std::str::FromStr;

use super::derive::*;
use super::profile::Author;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "head")]
pub struct OGMeta {
    #[html(selector = "meta[property='og:title']", attr = "content")]
    title: String,
    #[html(selector = "meta[property='og:title']", attr = "content")]
    owner: PageAuthor,
    #[html(selector = "meta[property='og:url']", attr = "content")]
    url: String,
    #[html(selector = "meta[property='og:image']", attr = "content")]
    image_url: Option<String>,
}

#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
struct PageAuthor(Author);
impl FromStr for PageAuthor {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.split(" -").map(str::trim);

        let (username, name) = (splitter.next(), splitter.next());
        Ok(Self(Author {
            username: username.ok_or(())?.into(),
            name: name.map(String::from),
            badge: None,
            avatar: None,
        }))
    }
}
