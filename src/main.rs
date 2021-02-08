mod parse;
use anyhow::*;
use args::{parse_args, Configuration};
use html5ever::{
    parse_document, tendril::TendrilSink, tokenizer::TokenizerOpts, tree_builder::TreeBuilderOpts,
    ParseOpts,
};
use io::{BufRead, Stdin, Stdout};
use parse::page::ParlerPage;
use parse::parser::*;
use serde_json::{self, to_writer};
use std::{borrow::{Borrow, BorrowMut}, fs::read, io::{self, Read}, path::{Path, PathBuf}};
use unhtml::{scraper::html, Element};
use walkdir::WalkDir;
use ProcessingError::FileIO;
mod args;

use anyhow::Result;
use crossbeam_channel::bounded;
use rayon::{prelude::*, spawn};
use thiserror::Error;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum InputStream {
    File(std::fs::File),
    Path(PathBuf),
    Stdin,
}
#[derive(Clone)]
struct NullWriter;

impl std::io::Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn read_buf_document<T>(source: &mut T) -> anyhow::Result<unhtml::scraper::Html>
where
    T: Read,
{
    let mut reader = io::BufReader::new(source);
    read_document(&mut reader)
}
fn read_document<T>(source: &mut T) -> anyhow::Result<unhtml::scraper::Html>
where
    T: Read,
{
    let doc = unhtml::scraper::Html::new_document();
    let parser = html5ever::parse_document(doc, ParseOpts::default());
    Ok(parser.from_utf8().read_from(source)?)
}

impl InputStream {
    fn read_document(&mut self) -> anyhow::Result<unhtml::scraper::Html> {
        match self {
            InputStream::File(f) => read_buf_document(f),
            InputStream::Stdin => {
                let stdin = std::io::stdin();
                let mut lock = stdin.lock();
                read_document(&mut lock)
            }
            InputStream::Path(p) => {
                
                let mut file = std::fs::File::open(p.as_path().borrow()).map_err(|e| ProcessingError::FileIO {
                    path: p.as_path().to_path_buf(),
                    source: e.into()
                })?;
                read_buf_document(&mut file)

            }
        }
    }
}

enum Message {
    Job((PathBuf, ParlerPage)),
    Stop,
}

impl From<(PathBuf, ParlerPage)> for Message {
    fn from(input: (PathBuf, ParlerPage)) -> Self {
        Message::Job(input)
    }
}

#[derive(Error, Debug)]
enum ProcessingError {
    #[error("err: failed to read {path}: {source}")]
    FileIO {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error("err: failed to parse html in {path}: {source}")]
    HTMLParseError {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error("err: found no match in {path}: {source}")]
    ParlerParseError {
        path: PathBuf,
        #[source]
        source: unhtml::Error,
    },
    #[error("error during processing: {0:?}")]
    Other(#[from] anyhow::Error),
    #[error("error during directory traversal: {0:?}")]
    Traversal(walkdir::Error),
}

impl From<walkdir::Error> for ProcessingError {
    fn from(e: walkdir::Error) -> Self {
        match e.path() {
            Some(p) => FileIO {
                path: p.to_path_buf(),
                source: e.into(),
            },
            None => ProcessingError::Traversal(e),
        }
    }
}
fn main() -> anyhow::Result<()> {
    let mut app = args::parse_args();
    let config = Configuration::from(app.clone().get_matches());

    let compact = config.compact();
    if !(config.path_count() > 0 || config.should_parse_stdin()) {
        app.print_long_help()?;
    }


    

    let (tx, rx) = bounded::<Message>(1024);

    let files = config
        .walk_paths()
        .filter_map(|v| match v {
            Ok(de) if de.file_type().is_file() => Some(Ok(de)),
            Ok(_) => None,
            Err(e) => Some(Err(ProcessingError::from(e))),
        })
        .map(|v| {
            v.and_then(|v| {
                let path = v.path();
                std::fs::File::open(path)
                    .map(|v| (path.to_path_buf(), v))
                    .map_err(|e| FileIO {
                        path: path.to_path_buf(),
                        source: e.into(),
                    })
            })
        })
        .filter_map(|v| match v {
            Ok((path, file)) => Some((path, InputStream::File(file))),
            Err(e) => {
                eprintln!("{}", e);
                
                None
            }
        })
        .par_bridge()
        .map(|(path, mut input): (PathBuf, InputStream)| {
            input
                .read_document()
                .and_then(|v| -> Result<(PathBuf, ParlerPage)> {
                    let p = path;
                    let sel = Selector::parse(":root").unwrap();
                    v.select(&sel)
                        .element()
                        .map_err(|e| {
                            ProcessingError::ParlerParseError {
                                path: p.clone(),
                                source: e,
                            }
                            .into()
                        })
                        .map(|v| (p.clone(), v))
                })
        })
        .map(|v| match v {
            Ok(job) => tx.send(job.into()).map_err(|source| {
                let job = source.into_inner();
                if let Message::Job((path, _)) = job {
                    anyhow::Error::from(ProcessingError::FileIO {
                        path,
                        source: anyhow!("channel send error"),
                    })
                } else {
                    unreachable!();
                }
            }),
            Err(e) => Err(e),
        });
    let rx = rx.clone();
    let writer = std::thread::spawn(move || {
        let mut stdout = io::stdout();
        let compact = compact;
        loop {
            let result = rx.recv();
            if let Ok(Message::Job((path, page))) = result {
                (if !compact {
                    serde_json::to_writer_pretty
                } else {
                    serde_json::to_writer
                })(stdout.borrow_mut(), &page)
                .unwrap();
                println!("");
                continue;
            } else if let Err(e) = result {
                eprintln!("closing {}", e);
            }
            break;
        }
    });

    files.for_each(|v| { /* TODO: log successful files and failed files */ });
    let res = tx.send(Message::Stop);
    writer.join().unwrap();
    res.map_err(|e| anyhow!(e))
}
