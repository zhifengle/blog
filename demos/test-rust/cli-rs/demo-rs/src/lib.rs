use std::error::Error;

use clap::{arg, ArgMatches, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    #[allow(dead_code)]
    paths: Vec<String>,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        Ok(Config {
            paths: matches
                .values_of("paths")
                .unwrap()
                .map(|s| s.to_owned())
                .collect(),
        })
    }
}

pub fn get_matches() -> ArgMatches {
    Command::new("demo-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust demo")
        .arg(arg!(paths: [PATH] ... "Search paths").default_value("-"))
        .get_matches()
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
