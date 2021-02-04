use clap::{crate_authors, crate_version, values_t_or_exit, App, Arg, ArgMatches, SubCommand};
use grep_cli::{is_readable_stdin, is_tty_stdout, stdout, stdout_buffered_line};
use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    io,
    iter::Rev,
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
).arg(Arg::with_name("output record delimiter")
.help("Define string outputted after each parsed file. Defaults to new line")
.long("delimiter")
.short("d")
.takes_value(true)
).arg(Arg::with_name("compact output")
.help("Output compact (single line) JSON. Defaults to true if stdin in not a terminal")
.long("compact")
.short("c")
).arg(Arg::with_name("fail file")
.help("Write failed paths to a file")
.takes_value(true)
.long("fail-log")
).arg(Arg::with_name("success file")
.help("Write successfully processed paths to a file")
.takes_value(true)
.long("success-log")
).arg(Arg::with_name("path file")
.help("Read paths from a file")
.takes_value(true)
.multiple(true)
.number_of_values(1)
.long("from-file")
)
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    paths: Vec<PathBuf>,
    compact_output: bool,
    recursive: bool,
    delimiter: String,
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
    pub fn walk_paths<'a>(&'a self) -> impl Iterator<Item = Result<DirEntry, walkdir::Error>> + 'a {
        let recursive = self.recursive;
        self.paths()
            .map(move |v| {
                let mut wd = WalkDir::new(v);
                if !recursive {
                    wd = wd.max_depth(1);
                }

                wd.into_iter().filter_entry(|v| !is_hidden(v))
            })
            .flatten()
    }

    pub fn should_parse_stdin(&self) -> bool {
        self.use_stdin
    }

    pub fn path_count(&self) -> usize {
        self.paths.len()
    }

    pub fn paths<'a>(&'a self) -> impl Iterator<Item = &'a PathBuf> {
        self.paths.iter()
    }
    pub fn compact(&self) -> bool {
        self.compact_output
    }
    pub fn recursive(&self) -> bool {
        self.recursive
    }
    pub fn delimiter(&self) -> &str {
        self.delimiter.as_ref()
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
            recursive: matches.is_present("recursive"),
            delimiter: matches
                .value_of("output record delimiter")
                .map_or("\n".into(), |v| v.into()),
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
        assert_eq!(config.delimiter, "\n");
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
        assert_eq!(config.delimiter, "\n");
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
