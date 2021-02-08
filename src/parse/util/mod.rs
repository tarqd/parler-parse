mod exists;
mod idfromsuffix;
mod idfromurl;

mod id;
mod skip;
mod untrimmedstring;

mod prelude {
    pub use super::super::derive::*;
    pub use super::super::parser::*;
    pub use std::str::FromStr;
}

pub use exists::*;
pub use id::*;
pub use idfromsuffix::*;
pub use idfromurl::*;
pub use skip::*;
pub use untrimmedstring::*;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! match_class {
    ($e:expr,  {  $pf:expr => $tf:expr, $($p:expr => $t:expr),*  }) => {
        {
            if $e.has_class($pf, CaseSensitivity::AsciiCaseInsensitive) {
                Ok($tf)
            } $(
                else if $e.has_class($p, CaseSensitivity::AsciiCaseInsensitive) {
                    Ok($t)
                }
            )+
            else {
                Err(().into())
            }
        }
    }
}
}
