pub mod media;
pub mod opengraph;
pub mod page;
pub mod post;
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
