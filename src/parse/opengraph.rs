use super::derive::*;
use super::parser::FromHtml;
#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "head")]
pub struct OGMeta {
    #[html(selector = "meta[property='og:title']", attr = "content")]
    title: Option<String>,
    #[html(selector = "meta[property='og:url']", attr = "content")]
    url: Option<String>,
    #[html(selector = "meta[property='og:image']", attr = "content")]
    image_url: Option<String>,
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
                title: Some("@username - name -".into()),
                url: Some("/post/id".into()),
                image_url: Some("https://images.parler.com/id_256".into()),
            }
        )
    }
}