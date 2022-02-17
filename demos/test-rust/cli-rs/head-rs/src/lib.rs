use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{arg, App, ArgMatches};

type MyResult<T> = Result<T, Box<dyn Error>>;

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
        // Err(val.into()); Err(Into::into(val))
        _ => Err(From::from(val)),
    }
}

// -------------------------
// 处理文件

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                // 0xA lf; 0xD cr
                // lines() 返回的行不包含换行符  ----> 使用 read_line()
                // for line in file.lines().take(config.lines) {
                //     let line = line?;
                //     println!("{}", line);
                // }
                if let Some(num_bytes) = config.bytes {
                    // 需要引入 io::Read 这样才能使用 file.take()
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    // 这里读取的一定的字节。读取整个文件是危险的，比如文件过大，导致内存爆了
                    let bytes_read = handle.read(&mut buffer)?;
                    // 有损的转换字符
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
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
