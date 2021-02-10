use super::prelude::*;
use super::ShouldSkip;
use base62::decode;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub id: Option<String>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub id_b62_dec: Option<u128>,
}

impl FromStr for Identifier {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier {
            id: Some(s.into()),
            id_b62_dec: decode(s).ok(),
        })
    }
}
