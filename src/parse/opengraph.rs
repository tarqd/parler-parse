use super::derive::*;
use super::parser::FromHtml;
use super::post::Author;
use std::str::FromStr;

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
struct PageAuthor(super::post::Author);
impl FromStr for PageAuthor {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.split(" -").map(str::trim);

        let (username, name) = (splitter.next(), splitter.next());
        Ok(Self(super::post::Author {
            username: username.ok_or(())?.into(),
            name: name.map(String::from),
            badge: None,
            avatar: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opengraph() {
        let input = r#"
    <!DOCTYPE html>
<html prefix="og: https://ogp.me/ns#">
<head>
    <title>@username - name - </title>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
    <meta name="description" content="Parleyed on Parler">

    <meta property="og:type" content="website" />
    <meta property="og:title" content="@username - name - " />
    <meta property="og:description" content="Parleyed on Parler" />
    <meta property="og:url" content="/post/id" />
    <meta property="og:image" content="https://images.parler.com/id_256" />
</head>
<body></body></html>"#;
        assert_eq!(
            OGMeta::from_html(input).unwrap(),
            OGMeta {
                title: "@username - name -".into(),
                url: "/post/id".into(),
                image_url: Some("https://images.parler.com/id_256".into()),
                owner: PageAuthor(Author {
                    username: "@username".into(),
                    name: Some("name".into()),
                    badge: None,
                    avatar: None,
                })
            }
        )
    }
}
