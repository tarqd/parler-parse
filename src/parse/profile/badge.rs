use super::prelude::*;
use std::result::Result;
use std::str::FromStr;
use url::Url;

// unhtml doesnt support derives from enums and I'm laazy
#[derive(FromText, Debug, Serialize, Deserialize, PartialEq)]
pub struct Badge(BadgeKind);

impl FromStr for Badge {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BadgeKind::from_str(s)?))
    }
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
