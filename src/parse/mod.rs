pub mod media;
pub mod opengraph;
pub mod page;
pub mod post;
pub mod profile;

pub mod util;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{to_writer, to_writer_pretty};
pub mod prelude {
    pub use super::media::*;
    pub use super::opengraph::*;
    pub use super::page::*;
    pub use super::parser::*;
    pub use super::post::*;
    pub use super::profile::*;
    pub use super::util::*;
    pub use super::{to_writer, to_writer_pretty};
    pub use super::{Deserialize, Serialize};
}

pub mod parser {
    pub use selectors::{attr::CaseSensitivity, Element};
    pub use unhtml::{
        scraper::{ElementRef, Html, Selector},
        ElemIter, Error, FromHtml, FromText, Result, Select,
    };
    #[macro_use]
    pub use super::util::macros::*;
}
mod derive {
    pub use serde::{Deserialize, Serialize};
    pub use unhtml_derive::{FromHtml, FromText};
}
