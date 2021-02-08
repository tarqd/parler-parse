use std::fmt::Display;
use super::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UntrimmedString(String);

impl unhtml::FromText for UntrimmedString {
    fn from_inner_text(select: ElemIter) -> unhtml::Result<Self> {
        select
            .next()
            .as_ref()
            .map(ElementRef::text)
            .map(unhtml::scraper::element_ref::Text::collect::<Vec<&str>>)
            .map(|v| v.concat())
            .map(String::from)
            .map(Self)
            .ok_or(().into())
    }

    fn from_attr(select: ElemIter, attr: &str) -> unhtml::Result<Self> {
        String::from_attr(select, attr).map(Self)
    }
}
impl From<UntrimmedString> for String {
    fn from(value: UntrimmedString) -> Self {
        value.0
    }
}
impl AsRef<str> for UntrimmedString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for UntrimmedString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use unhtml::scraper::Html;
    #[test]
    fn untrimmed_string() {
        use unhtml::Text;
        let test = r#"<p><a href="/profile/RudyG/posts" class="at">@RudyG</a> <br><a href="/profile/JennaEllisEsq/posts" class="at">@JennaEllisEsq</a> <br><a href="/profile/SidneyPowell/posts" class="at">@SidneyPowell</a></p>"#;
        let doc = Html::parse_fragment(&test);
        let sel = unhtml::scraper::Selector::parse("p").unwrap();
        let res: UntrimmedString = doc.select(&sel).inner_text().unwrap();
        assert_eq!("@RudyG @JennaEllisEsq @SidneyPowell", res.to_string());
    }
}