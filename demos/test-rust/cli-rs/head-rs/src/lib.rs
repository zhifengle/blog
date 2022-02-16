use std::{error::Error, fs::File};

use clap::{arg, App, ArgMatches};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

impl Config {
    pub fn new(files: Vec<String>, lines: usize, bytes: Option<usize>) -> Self {
        Config {
            files,
            lines,
            bytes,
        }
    }
    // String::from   From 特征
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        let lines = matches
            .value_of("lines")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal line count -- {}", e))?;

        let bytes = matches
            .value_of("bytes")
            .map(parse_positive_int)
            // Option 包裹 ----> Result 包裹
            .transpose()
            .map_err(|e| format!("illegal byte count -- {}", e))?;

        let files: Vec<_> = matches
            .values_of("files")
            .unwrap()
            .map(|s| s.to_owned())
            .collect();

        Ok(Config {
            files,
            lines: lines.unwrap(),
            bytes,
        })
    }
}

pub fn get_matches() -> ArgMatches {
    App::new("head-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust head")
        .arg(arg!(files: [FILES] ... "input file(s)").default_value("-"))
        .arg(
            arg!(lines: -n --lines [LINES] "NUM lines")
                // .takes_value(true)
                .default_value("10"),
        )
        .arg(
            arg!(bytes: -c --bytes [BYTES] "NUM bytes")
                // 教程里面有这项配置，暂时不知道作用。不写也能运行
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .get_matches()
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // 返回值限定了 usize，所有没有写 turbofish
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        // 将字符串转化成 err ??
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
