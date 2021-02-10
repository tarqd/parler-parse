use anyhow::Context;
use clap::{crate_authors, crate_version, values_t_or_exit, App, Arg, ArgMatches, SubCommand};
use grep_cli::{is_readable_stdin, is_tty_stdout, stdout, stdout_buffered_line};
use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, BufRead, BufReader},
    iter::{self, Rev},
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    vec,
};
use walkdir::{self, DirEntry, WalkDir};
pub fn parse_args<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("parler-parse")
    .version(crate_version!())
    .author(crate_authors!())
    .arg(Arg::with_name("path")
      .help("HTML File(s) or directory of HTML File(s) to parse")
      .long("path")
      .short("p")
      .multiple(true)
      .index(1)
      
  ).arg(Arg::with_name("recursive")
  .help("Recursively search directories")
  .long("recursive")
  .short("r")
).arg(Arg::with_name("compact output")
.help("Output compact (single line) JSON. Defaults to true if stdin in not a terminal")
.long("compact")
.short("c")
).arg(Arg::with_name("fail file")
.help("Write failed paths to a file")
.takes_value(true)
.long("fail-log")
.number_of_values(1)
).arg(Arg::with_name("success file")
.help("Write successfully processed paths to a file")
.takes_value(true)
.number_of_values(1)
.long("success-log")
).arg(Arg::with_name("path file")
.help("Read paths from a file")
.takes_value(true)
.multiple(true)
.long("paths-from-file")
)
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    paths: Vec<PathBuf>,
    success_path: Option<PathBuf>,
    path_file: Option<PathBuf>,
    fail_path: Option<PathBuf>,
    compact_output: bool,
    recursive: bool,
    use_stdin: bool,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

impl Configuration {
    pub fn walk_paths<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Result<DirEntry, walkdir::Error>> + Send> {
        let recursive = self.recursive;
        let paths = self.paths();
        let paths_from_file = match self.path_file() {
            Some(file) => Some(
                File::open(file)
                    .map(BufReader::new)
                    .map(|v| {
                        v.lines().map(|v| {
                            v.map(|v| {
                                PathBuf::from(OsStr::from_bytes(
                                    grep_cli::unescape(v.as_str()).as_slice(),
                                ))
                            })
                        })
                    })
                    .context("failed to read line file"),
            ),
            None => None,
        };
        if let Some(Err(e)) = paths_from_file {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        let process = move |v| {
            let mut wd = WalkDir::new(v);
            if !recursive {
                wd = wd.max_depth(0);
            }

            wd.into_iter().filter_entry(|v| !is_hidden(v))
        };

        if let Some(Ok(path_file)) = paths_from_file {
            Box::new(
                path_file
                    .filter_map(|v| v.ok())
                    .map(move |v| process(v))
                    .flatten(),
            )
        } else {
            return Box::new(self.paths().map(move |v| process(v)).flatten());
        }
    }

    pub fn should_parse_stdin(&self) -> bool {
        self.use_stdin
    }

    pub fn path_count(&self) -> usize {
        self.paths.len()
    }

    pub fn paths(&self) -> impl Iterator<Item = PathBuf> {
        self.paths.clone().into_iter()
    }
    pub fn compact(&self) -> bool {
        self.compact_output
    }
    pub fn recursive(&self) -> bool {
        self.recursive
    }
    pub fn path_file(&self) -> Option<&PathBuf> {
        self.path_file.as_ref()
    }
    pub fn fail_path(&self) -> Option<&PathBuf> {
        self.fail_path.as_ref()
    }
    pub fn success_path(&self) -> Option<&PathBuf> {
        self.success_path.as_ref()
    }
}

impl<'a> From<clap::ArgMatches<'a>> for Configuration {
    fn from(matches: clap::ArgMatches<'a>) -> Self {
        let mut found_stdin_path = false;
        let paths: Vec<PathBuf> = matches
            .values_of_os("path")
            .map(|iter| {
                iter.filter_map(|v| {
                    if v == "-" {
                        found_stdin_path = true;
                        None
                    } else {
                        Some(PathBuf::from(v))
                    }
                })
                .collect()
            })
            .unwrap_or_else(|| vec![]);

        Self {
            paths,
            use_stdin: (!matches.is_present("path") && is_readable_stdin() || found_stdin_path),
            compact_output: (matches.is_present("compact output") || !is_tty_stdout()),
            fail_path: matches.value_of("fail file").map(|v| PathBuf::from(v)),
            success_path: matches.value_of("success file").map(|v| PathBuf::from(v)),
            recursive: matches.is_present("recursive"),
            path_file: matches.value_of("path file").map(PathBuf::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arg_parser_flags() -> clap::Result<()> {
        let app = parse_args();
        let config = Configuration::from(app.get_matches_from_safe(vec!["test", "-r", "-c"])?);
        assert_eq!(config.recursive, true);
        assert_eq!(config.compact_output, true);
        assert_eq!(config.paths.len(), 0);
        assert_eq!(config.use_stdin, false);
        Ok(())
    }
    #[test]
    fn test_arg_parser_defaults() -> clap::Result<()> {
        let app = parse_args();
        let config = Configuration::from(app.get_matches_from_safe(vec!["test", "-"])?);
        assert_eq!(config.recursive, false);
        assert_eq!(config.compact_output, !is_tty_stdout());
        assert_eq!(config.paths.len(), 0);

        assert_eq!(config.use_stdin, true);
        Ok(())
    }
    #[test]
    fn test_arg_parser_positional() -> clap::Result<()> {
        let app = parse_args();
        let config =
            Configuration::from(app.get_matches_from_safe(vec!["test", "-c", "/test", "/test2"])?);
        assert_eq!(
            config.paths,
            vec![PathBuf::from("/test"), PathBuf::from("/test2")]
        );
        assert_eq!(config.compact_output, true);
        Ok(())
    }
}
