pub mod opengraph;
pub mod post;
pub mod page;
pub mod media;
pub mod parser {
    pub use unhtml::{
        FromHtml, FromText, ElemIter, Select,
        scraper::{
            Html, ElementRef, Selector,
        }
    };
    pub use selectors::{
        Element,
        attr::{CaseSensitivity}
    };

}

pub mod derive {
    pub use unhtml_derive::{FromHtml, FromText};
    pub use serde::{Serialize, Deserialize};
}