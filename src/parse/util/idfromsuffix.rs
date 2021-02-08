use super::prelude::*;
use std::result::Result;
#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
pub struct IDFromSuffix(String);

impl FromStr for IDFromSuffix {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.rsplit('-')
            .next()
            .map(|v| Self(v.into()))
            .ok_or(().into())
    }
}