use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{arg, App};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        // 向右缩进
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!()
                        }
                    } else {
                        println!("{}", line)
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("car-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust cat")
        .arg(arg!(files: [FILES] ... "input file(s)").default_value("-"))
        .arg(arg!(number: -n --number "Number lines").conflicts_with("number_nonblank"))
        // 中间有横线需要用引号包裹
        .arg(arg!(number_nonblank: -b --"number-nonblank" "Number non-blank lines"))
        .get_matches();
    let files: Vec<_> = matches
        .values_of("files")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();
    Ok(Config {
        files,
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// 编译时不知道大小，需要使用 Box
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
