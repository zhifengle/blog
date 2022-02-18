use crate::Extract::*;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

use clap::{arg, ArgMatches, Command};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        let delimiter = matches.value_of("delimiter").unwrap();
        let delim_bytes = delimiter.as_bytes();
        if delim_bytes.len() != 1 {
            return Err(From::from(format!(
                "--delim \"{}\" must be a single byte",
                delimiter
            )));
        }
        let fields = matches.value_of("fields").map(parse_pos).transpose()?;
        let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;
        let chars = matches.value_of("chars").map(parse_pos).transpose()?;
        let extract = if let Some(field_pos) = fields {
            Fields(field_pos)
        } else if let Some(byte_pos) = bytes {
            Bytes(byte_pos)
        } else if let Some(char_pos) = chars {
            Chars(char_pos)
        } else {
            return Err(From::from("Must have --fields, --bytes, or --chars"));
        };
        Ok(Config {
            files: matches
                .values_of("files")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
            delimiter: *delim_bytes.first().unwrap(),
            extract,
        })
    }
}

pub fn get_matches() -> ArgMatches {
    Command::new("cut-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust cut")
        .arg(arg!(files: [FILE] ... "Input file(s)").default_value("-"))
        .arg(
            arg!(bytes: -b --bytes [BYTES] "Selected bytes")
                .conflicts_with_all(&["chars", "fields"]),
        )
        .arg(
            arg!(chars: -c --chars [CHARS] "Selected characters")
                .conflicts_with_all(&["bytes", "fields"]),
        )
        .arg(arg!(delimiter: -d --delim [DELIMITER] "Field delimiter").default_value("\t"))
        .arg(
            arg!(fields: -f --fields [DELIMITER] "Selected fields")
                .conflicts_with_all(&["bytes", "chars"]),
        )
        .get_matches()
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// &PositionList x ----> &[_]
fn extract_chars(line: &str, pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    let mut selected: Vec<char> = vec![];
    for range in pos.iter().cloned() {
        // range 要被消耗，所以 clone(); 或者 pos.iter().cloned()
        // for i in range {
        //     if let Some(val) = chars.get(i) {
        //         selected.push(*val);
        //     }
        // }
        selected.extend(range.filter_map(|i| chars.get(i)));
    }
    selected.iter().collect()
}
#[allow(dead_code)]
fn extract_chars2(line: &str, pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    pos.iter()
        .cloned()
        // flat_map ==  map + flatten
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        // 展开双重包裹的 Iterator
        // .flatten()
        .collect()
}

fn extract_bytes(line: &str, pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = pos
        .iter()
        .cloned()
        // 需要复制一份
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields(record: &StringRecord, pos: &[Range<usize>]) -> Vec<String> {
    pos.iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .map(String::from)
        .collect()
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", &config);
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                // println!("Opened {}", filename);
                match &config.extract {
                    // cargo run -- -d , -f 1,3 tests/inputs/books.csv
                    Fields(pos) => {
                        let mut reader = ReaderBuilder::new()
                            .delimiter(config.delimiter)
                            .has_headers(false)
                            .from_reader(file);
                        let mut writter = WriterBuilder::new()
                            .delimiter(config.delimiter)
                            .from_writer(io::stdout());
                        for record in reader.records() {
                            let record = record?;
                            writter.write_record(extract_fields(&record, pos))?;
                        }
                    }
                    Bytes(pos) => {
                        for line in file.lines() {
                            println!("{}", extract_bytes(&line?, pos));
                        }
                    }
                    Chars(pos) => {
                        for line in file.lines() {
                            println!("{}", extract_chars(&line?, pos));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);
    input
        .starts_with("+")
        // +开头报错
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                // 不为 0 的正整数
                .parse::<NonZeroUsize>()
                // 索引减一
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(",")
        .into_iter()
        .map(|val| {
            // 先解析单个数字，如果出错尝试使用正则匹配
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                range_re.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        return Err(format!(
                            "First number in range ({}) must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1,
                        ));
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        // 转换 parse_index 的 String 信息为 Error
        .map_err(From::from)
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use csv::StringRecord;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"",);

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"",);

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-+2\"",);

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-a\"",);

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a-1\"",);

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 1..2, 4..5]), "áb".to_string());
    }
    #[test]
    fn test_extract_bytes() {
        // 这里的 á 分成字节后，解析不出字符
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    }
    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(extract_fields(&rec, &[0..1, 2..3]), &["Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }
}
