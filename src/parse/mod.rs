use unhtml::scraper::ElementRef;
use unhtml::ElemIter;
use url::{ParseError as UrlParseError, Url};
pub mod media;
pub mod opengraph;
pub mod page;
pub mod post;

pub use types::UntrimmedString;

mod types {
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display};
    use unhtml::scraper::ElementRef;
    use unhtml::ElemIter;
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
}

pub mod parser {
    pub use selectors::{attr::CaseSensitivity, Element};
    pub use unhtml::{
        scraper::{ElementRef, Html, Selector},
        ElemIter, FromHtml, FromText, Select,
    };
}

pub mod derive {
    pub use serde::{Deserialize, Serialize};
    pub use unhtml_derive::{FromHtml, FromText};
}
