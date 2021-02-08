pub struct FileMeta {
    full_path: PathBuf,
    file_name: PathBuf,
    create_dt: u64,
    modified_dt: u64
}

pub enum ScrapeSource {
    Wayback, CommonCrawl, Other(String)
}

pub enum InputKind {
    HTML, WARC
}

impl std::default::Default for InputKind{
        fn default() -> Self { Kind::HTML }
}

pub struct ScrapeMeta {
    source: ScrapeSource,
    file: FileMeta,
    parse_dt: Option<u64>,
    kind: InputKind
}