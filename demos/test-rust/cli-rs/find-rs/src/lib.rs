// 这样就可以使用 Dir, File, Link 了
use crate::EntryType::*;

use std::error::Error;

use clap::{arg, ArgMatches, Command};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

// clap 3.1 使用 Command 替换 App

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> MyResult<Self> {
        // 例子用  values_of_lossy 需要设置 Arg invalid_utf8
        let names = matches
            .values_of("names")
            .map(|vals| {
                vals.into_iter()
                    .map(|name| {
                        Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();

        let entry_types = matches
            .values_of("types")
            .map(|vals| {
                vals.into_iter()
                    .map(|val| match val {
                        "d" => Dir,
                        "f" => File,
                        "l" => Link,
                        _ => unreachable!("Invalid type"),
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(Config {
            // paths: matches
            //     .values_of("paths")
            //     .map(|vals| vals.into_iter().map(|s| s.to_owned()).collect())
            //     .unwrap_or_default(),
            paths: matches
                .values_of("paths")
                .unwrap()
                .map(|s| s.to_owned())
                .collect(),
            names,
            entry_types,
        })
    }
}

pub fn get_matches() -> ArgMatches {
    Command::new("find-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust find")
        .arg(arg!(paths: [PATH] ... "Search paths").default_value("."))
        .arg(arg!(names: -n --name [NAME] ... "Name"))
        .arg(arg!(types: -t --type [TYPE] "Entry type").possible_values(&["f", "d", "l"]))
        .get_matches()
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path_is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };
    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };
    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
