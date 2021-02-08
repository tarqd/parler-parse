use super::prelude::*;
use std::result::Result;
use super::id::Identifier;

#[derive(FromText, Debug, PartialEq, Serialize, Deserialize)]
pub struct IDFromSuffix(Identifier);

impl From<IDFromSuffix> for Identifier {
    fn from(v: IDFromSuffix) -> Self {
        v.0
    }
}
impl FromStr for IDFromSuffix {
    type Err = unhtml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.rsplit('-')
            .next()
            .map(|v| Identifier::from_str(v).map(Self).ok())
            .flatten()
            .ok_or(().into())
    
            

    }
}