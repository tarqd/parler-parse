use super::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ElementExists(bool);

impl FromHtml for ElementExists {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        Ok(Self(select.next().is_some()))
    }
}
