use std::{ffi::OsStr, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use anyhow::*;
use super::derive::*;
use super::util::ShouldSkip;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileMeta {
    #[serde(serialize_with = "serialize_path")]
    pub path: PathBuf,
    create_dt: Option<u64>,
    modified_dt: Option<u64>
}

fn serialize_path<S>(v : &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    serializer.serialize_str(grep_cli::escape_os(v.as_os_str()).as_str())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputKind {
    HTML, WARC
}

impl std::default::Default for InputKind{
        fn default() -> Self { InputKind::HTML }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScrapeMeta {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub source: Option<String>,
    pub sha1: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    #[serde(flatten)]
    pub file: Option<FileMeta>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub parse_dt: Option<u64>,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ParseOutput {
    #[serde(rename="__meta")]
    pub meta : ScrapeMeta,
    #[serde(flatten)]
    pub page : super::page::ParlerPage

}

pub struct OutputBuilder {
    path: PathBuf,
    create_dt: Option<u64>,
    modify_dt: Option<u64>,
    source: Option<String>,
    sha1: Option<String>,
    parse_dt: Option<u64>,
    kind: InputKind
}

macro_rules! impl_optional_builder_methods {
    ($($i:ident : $t:ty),* ) => {
        $(
            pub fn $i<'a>(&'a mut self, value : $t) -> &'a mut Self {
                self.$i = value;
                self
            }
        )*
    };
}
macro_rules! impl_required_builder_methods {
    ($($i:ident : $t:ty),* ) => {
        $(
            pub fn $i<'a>(&'a mut self, value : $t) -> &'a mut Self {
                self.$i = Some(value);
                self
            }
        )*

    };
}

fn to_ts(st: SystemTime) -> Option<u64> {
    st.duration_since(UNIX_EPOCH).ok().map(|v| v.as_secs())
}
impl OutputBuilder {
    pub fn new(kind: InputKind, path: PathBuf) -> Self {
        Self {
            path: path,
            source: None,
            sha1: None,
            parse_dt: None,
            create_dt: None,
            modify_dt: None,
            kind,
        }
    }
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    pub fn entry<'a>(&'a mut self, entry: &walkdir::DirEntry) -> &'a mut Self {
        match entry.metadata().ok() {
            Some(meta) => {
                self.create_dt = meta.created().ok().map(to_ts).flatten();
                self.modify_dt = meta.modified().ok().map(to_ts).flatten();
            },
            _ => { self.create_dt = None; self.modify_dt = None; }
        };
        
        self
    }
    impl_optional_builder_methods!(
        source: Option<String>,
        create_dt: Option<u64>,
        modify_dt: Option<u64>
    );
    impl_required_builder_methods!(
        sha1: String
    );
    pub fn build(self, page: super::page::ParlerPage) -> Result<ParseOutput> {
        Ok(ParseOutput {
            meta : ScrapeMeta {
                source: self.source,
                sha1: self.sha1.ok_or_else(|| anyhow!("missing sha1"))?,
                file: if self.path.to_string_lossy().ne("-") {
                    Some(FileMeta {
                        path: self.path,
                        create_dt: self.create_dt,
                        modified_dt: self.modify_dt,
                        
                    })
                } else { None },
                parse_dt: to_ts(SystemTime::now())
            },
            page
            
        })
    }
}