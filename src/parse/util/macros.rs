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