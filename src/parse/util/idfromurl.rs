use super::prelude::*;
use std::result::Result;
use super::id::Identifier;

#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
pub struct IDFromUrl(Identifier);
impl From<IDFromUrl> for String {
    fn from(v: IDFromUrl) -> Self {
        v.0.id.unwrap_or_default()
    }
}
impl From<IDFromUrl> for Identifier {
    fn from(v: IDFromUrl) -> Self {
        v.0
    }
}
impl FromStr for IDFromUrl {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base = url::Url::parse("https://parler.com/").unwrap();
        let options = url::Url::options().base_url(Some(&base));

        let url = options.parse(s).or_else(|e| Err(()))?;
        let domain = url.domain().ok_or(())?;

        if domain.ends_with(".parler.com") {
            let path = std::path::Path::new(url.path());
            let id = path.file_stem().ok_or(())?.to_str().ok_or(())?;

            return id
                .split('_')
                .next()
                .map(|s| Identifier::from_str(s).map(Self).ok())
                .flatten()
                .ok_or(().into());
        }
        Err(().into())
    }
}