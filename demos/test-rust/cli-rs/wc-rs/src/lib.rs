use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{arg, ArgMatches, Command};

// clap 3.1 使用 Command 替换 App

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_bytes: usize,
    num_chars: usize,
    num_words: usize,
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_words = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_bytes,
        num_words,
        num_chars,
    })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    bytes: bool,
    chars: bool,
    words: bool,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        let mut lines = matches.is_present("lines");
        let mut words = matches.is_present("words");
        let mut bytes = matches.is_present("bytes");
        let chars = matches.is_present("chars");
        if [lines, words, bytes, chars].iter().all(|v| v == &false) {
            lines = true;
            words = true;
            bytes = true;
        }

        let files: Vec<_> = matches
            .values_of("files")
            .unwrap()
            .map(|s| s.to_owned())
            .collect();

        Ok(Config {
            files,
            lines,
            bytes,
            chars,
            words,
        })
    }
}

pub fn get_matches() -> ArgMatches {
    Command::new("wc-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust wc")
        .arg(arg!(files: [FILES] ... "input file(s)").default_value("-"))
        .arg(arg!(lines: -l --lines "Show line count"))
        .arg(arg!(bytes: -c --bytes "Show byte count"))
        .arg(arg!(words: -w --words "Show word count"))
        .arg(arg!(chars: -m --chars "Show character count").conflicts_with("bytes"))
        .get_matches()
}

// -------------------------
// 处理文件

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn t_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
    #[test]
    fn t_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
