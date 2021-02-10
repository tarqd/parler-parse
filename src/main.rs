mod parse;
use anyhow::*;
use args::{Configuration};
use html5ever::{ParseOpts, parse_document, tendril::TendrilSink, tokenizer::TokenizerOpts, tree_builder::TreeBuilderOpts};
use io::{BufRead, BufWriter, Stdin, Stdout};
use parse::page::ParlerPage;
use parse::parser::*;
use serde_json::{self, to_writer};
use std::{
    borrow::{Borrow, BorrowMut},
    fs::read,
    io::{self, Read},
    path::{Path, PathBuf},
};
use std::{cell::RefCell, fmt};
use unhtml::{scraper::html, Element};
use walkdir::WalkDir;
use ProcessingError::FileIO;
mod args;

use anyhow::Result;
use crossbeam_channel::{bounded, SendError};
use rayon::{prelude::*, spawn};
use std::io::Write;
use std::sync::{Arc, Mutex};
use thiserror::Error;

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
                let mut file = std::fs::File::open(p.as_path().borrow()).map_err(|e| {
                    ProcessingError::FileIO {
                        path: p.as_path().to_path_buf(),
                        source: e.into(),
                    }
                })?;
                read_buf_document(&mut file)
            }
        }
    }
}

enum Message {
    Job((PathBuf, ParlerPage)),
    ErrorLog(PathBuf),
    Stop,
}

impl From<(PathBuf, ParlerPage)> for Message {
    fn from(input: (PathBuf, ParlerPage)) -> Self {
        Message::Job(input)
    }
}

#[derive(Error, Debug)]
enum ProcessingError {
    #[error("failed to read {path}: {source}")]
    FileIO {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error("failed to parse html in {path}: {source}")]
    HTMLParseError {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error("failed to send job to channel: {path}")]
    JobSendError { path: PathBuf },
    #[error("found no match in {path}: {source}")]
    ParlerParseError {
        path: PathBuf,
        #[source]
        source: unhtml::Error,
    },
    #[error("processing error: {0:?}")]
    Other(#[from] anyhow::Error),
    #[error("error during directory traversal: {0:?}")]
    Traversal(walkdir::Error),
}

impl ProcessingError {
    fn path(&self) -> Option<&PathBuf> {
        match self {
            ProcessingError::ParlerParseError { path, .. }
            | ProcessingError::HTMLParseError { path, .. }
            | ProcessingError::JobSendError { path }
            | ProcessingError::FileIO { path, .. } => Some(path),
            _ => None,
        }
    }
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

impl From<crossbeam_channel::SendError<Message>> for ProcessingError {
    fn from(message: crossbeam_channel::SendError<Message>) -> Self {
        match message.into_inner() {
            Message::Job((path, _)) | Message::ErrorLog(path) => {
                ProcessingError::JobSendError { path }
            }
            Message::Stop => {
                ProcessingError::Other(anyhow!("failed to send stop message to channel"))
            }
        }
    }
}

type BufFile = io::BufWriter<std::fs::File>;
fn main() -> anyhow::Result<()> {
    let mut app = args::parse_args();
    let config = Configuration::from(app.clone().get_matches());

    let compact = config.compact();
    if !(config.path_count() > 0 || config.should_parse_stdin()) {
        app.print_long_help()?;
    }
    let fail_log = config.fail_path().map(|v| -> Result<BufFile> {
        std::fs::File::create(v)
            .map_err(|e| {
                anyhow::Error::from(e).context(format!(
                    "failed to open failure log file {}",
                    grep_cli::escape_os(v.as_os_str())
                ))
            })
            .map(BufWriter::new)
    });
    let success_log = config.success_path().map(|v| -> Result<BufFile> {
        std::fs::File::create(v)
            .map_err(|e| {
                anyhow::Error::from(e).context(format!(
                    "failed to open failure log file {}",
                    grep_cli::escape_os(v.as_os_str())
                ))
            })
            .map(BufWriter::new)
    });
    if let Some(Err(e)) = success_log {
        bail!(e);
    }
    if let Some(Err(e)) = fail_log {
        bail!(e);
    }
    let should_parse_stdin = config.should_parse_stdin();

    let send_errors = fail_log.is_some();

    let (tx, rx) = crossbeam_channel::unbounded::<Message>();

    let paths = config.walk_paths();

    let files = std::iter::once_with(|| {
        if should_parse_stdin {
            Some(Ok((PathBuf::from("-"), InputStream::Stdin)))
        } else {
            None
        }
    })
    .flatten()
    .par_bridge()
    .chain(
        config
            .walk_paths()
            .par_bridge()
            .filter_map(|v| match v {
                Ok(de) if de.file_type().is_file() => Some(Ok(de)),
                Ok(_) => None,
                Err(e) => Some(Err(ProcessingError::from(e))),
            })
            .map(|v| {
                v.and_then(|v| {
                    let path = v.path();
                    std::fs::File::open(path)
                        .map(|v| (path.to_path_buf(), InputStream::File(v)))
                        .map_err(|e| FileIO {
                            path: path.to_path_buf(),
                            source: e.into(),
                        })
                })
            }),
    )
    .map(|res| {
        res.and_then(|(path, mut input)| {
            input
                .read_document()
                .map_err(|e| ProcessingError::HTMLParseError {
                    path: path.clone(),
                    source: e,
                })
                .and_then(
                    |v: Html| -> Result<(PathBuf, ParlerPage), ProcessingError> {
                        let p = path;
                        let sel = Selector::parse(":root").unwrap();
                        v.select(&sel)
                            .element()
                            .map_err(|e| ProcessingError::ParlerParseError {
                                path: p.clone(),
                                source: e,
                            })
                            .map(|v| (p, v))
                    },
                )
                .map(|v| Message::from(v))
        })
    });

    let writer = std::thread::spawn(move || -> Result<()> {
        let mut stdout = io::stdout();
        let mut success_log = match success_log {
            Some(Ok(l)) => Some(l),
            _ => None,
        };
        let mut fail_log = match fail_log {
            Some(Ok(l)) => Some(l),
            _ => None,
        };

        loop {
            let result = rx.recv()?;
            match result {
                Message::Job((path, page)) => {
                    (if !compact {
                        serde_json::to_writer_pretty
                    } else {
                        serde_json::to_writer
                    })(stdout.borrow_mut(), &page)
                    .context("error while writing output")?;

                    println!("");
                    if let Some(log) = success_log.borrow_mut() {
                        writeln!(log, "{}", grep_cli::escape_os(path.as_os_str()))
                            .context("error while writing to success log")?;
                    }
                    continue;
                }
                Message::ErrorLog(ref path) => {
                    if let Some(fail_log) = fail_log.borrow_mut() {
                        writeln!(fail_log, "{}", grep_cli::escape_os(path.as_os_str()))
                            .context("error while writing to fail log")?;
                    }
                }

                Message::Stop => break,
            }
        }
        fail_log.and_then(|mut v| v.flush().ok());
        success_log.and_then(|mut v| v.flush().ok());
        Ok(())
    });

    let _ = files
        .map(|v| v.map(|v| tx.send(v).map_err(ProcessingError::from)))
        .all(|result| {
            if let Err(e) = result {
                eprintln!("{}", e);
                if let (true, Some(path)) = (send_errors, e.path()) {
                    if let Err(_) = tx.send(Message::ErrorLog(path.into())) {
                        eprintln!("{}", anyhow!("failure while sending error log job"));
                        return false;
                    }
                }

                if let ProcessingError::JobSendError { .. } = e {
                    return false;
                }
            }
            true
        });

    let res = tx.send(Message::Stop);
    writer.join().unwrap();
    res.map_err(|e| anyhow!(e))
}
