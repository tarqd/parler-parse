pub mod media;
pub mod opengraph;
pub mod page;
pub mod post;
pub mod profile;

pub mod util;
pub use serde::{Serialize, Deserialize};
pub use serde_json::{to_writer, to_writer_pretty};
pub mod prelude {
    pub use super::media::*;
    pub use super::opengraph::*;
    pub use super::page::*;
    pub use super::post::*;
    pub use super::profile::*;
    pub use super::util::*;
    pub use super::{to_writer_pretty, to_writer};
    pub use super::{Serialize, Deserialize};
    pub use super::parser::*;
}


pub mod parser {
    pub use selectors::{attr::CaseSensitivity, Element};
    pub use unhtml::{
        scraper::{ElementRef, Html, Selector},
        ElemIter, FromHtml, FromText, Select, Result, Error
    };
    #[macro_use]
    pub use super::util::macros::*;
}
mod derive {
    pub use serde::{Deserialize, Serialize};
    pub use unhtml_derive::{FromHtml, FromText};
}

