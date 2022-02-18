use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use clap::{arg, ArgMatches, Command};

// clap 3.1 使用 Command 替换 App

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        let count = matches.is_present("count");

        let in_file = matches.value_of("in_file").unwrap();
        let out_file = matches.value_of("out_file").map(String::from);

        Ok(Config {
            count,
            in_file: in_file.to_string(),
            out_file,
        })
    }
}

pub fn get_matches() -> ArgMatches {
    Command::new("uniq-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust uniq")
        .arg(arg!(in_file: [IN_FILE] "Input file").default_value("-"))
        .arg(arg!(out_file: [OUT_FILE] "Ouput file"))
        .arg(arg!(count: -c --count "Show counts"))
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

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    // io::Write
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    // 使用了闭包，闭包可以访问内部的变量 config。使用函数不能访问 config
    // let print = |count: u64, text: &str| {
    //     if count > 0 {
    //         if config.count {
    //             print!("{:>4} {}", count, text);
    //         } else {
    //             print!("{}", text);
    //         }
    //     };
    // };
    // 必须声明 mut print; 里面修改了 out_file
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        // print!("{}", line);
        line.clear();
    }
    print(count, &previous)?;
    Ok(())
}
