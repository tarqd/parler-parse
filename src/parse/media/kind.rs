use super::prelude::*;

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MediaKind {
    Video,
    Audio,
    Image,
    Article,
    Basic,
    IframeEmbed,
    Website,
}

impl FromHtml for MediaKind {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let elem = select.next().ok_or(())?;
        let elem = elem.value();

        match_class!(elem, {
            "mc-video--container" => MediaKind::Video,
            "mc-image--container" => MediaKind::Image,
            "mc-basic--container" => MediaKind::Basic,
            "mc-article--container" => MediaKind::Article,
            "mc-website--container" => MediaKind::Website,
            "mc-iframe-embed--container" => MediaKind::IframeEmbed,
            "mc-audio--container" => MediaKind::Audio
        })
    }
}
