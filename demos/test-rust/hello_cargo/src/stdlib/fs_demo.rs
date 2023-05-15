#![allow(unused_imports)]
#![allow(dead_code)]

// https://doc.rust-lang.org/std/fs/index.html
// https://doc.rust-lang.org/std/fs/struct.File.html

use std::fs;
use std::io::prelude::*;
use std::{error::Error as StdError, fs::OpenOptions};
use std::{
    fs::File,
    io::{BufReader, Read},
};

// open create write; check

// 判断文件是否存在
// https://stackoverflow.com/questions/32384594/how-to-check-whether-a-path-exists
// https://programming-idioms.org/idiom/144/check-if-file-exists/1988/rust

// 这种方式是更方便。
#[test]
fn t_read() -> std::io::Result<()> {
    let toml = "Cargo.toml";
    // Vec<u8>
    let mut file = fs::read(toml)?;
    // String
    let contents = fs::read_to_string(toml)?;

    // 判断是否存在
    let b = std::path::Path::new(toml).exists();

    Ok(())
}

#[test]
fn t_create() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

#[test]
fn t_open() {
    let mut file = File::open("poem.txt").expect("no such file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

#[test]
fn t_buf_reader() {
    let file = File::open("poem.txt").expect("no such file");
    let mut buf_reader = BufReader::new(file);
    // 使用 serde_json 来解析
    // serde_json::from_reader(rdr)
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

// -------------------------------------------------

// serde = { version = "1", features = ["derive"]}
// serde_json = "1"

// fn get_config(filename: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
fn get_config(filename: &str) -> Result<(), Box<dyn StdError>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        contents = "{}".to_string();
    }
    println!("{}", contents);
    // let contents = fs::read_to_string(filename)?;
    // let v: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(())
}

// 写入文件
fn write_config(filename: &str, contents: &str) {
    let _r = fs::write(&filename, contents);
}

fn find_txt_files(folder_path: &str) -> Vec<String> {
    let mut txt_files = Vec::new();
    let folder = fs::read_dir(folder_path).expect("Failed to read directory");

    for file in folder {
        let file_path = file.expect("Failed to get file path").path();
        if let Some(extension) = file_path.extension() {
            if extension == "txt" {
                txt_files.push(file_path.to_str().unwrap().to_owned());
            }
        }
    }

    txt_files
}

// -------------------------------------------------

// notes: 编译时不知道大小，需要使用 Box
fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// notes: 传入字符串或者PathBuf
fn some_path_fn<Q>(base_str: &str, name: Q) -> std::io::Result<()>
where
    Q: AsRef<Path>,
{
    Ok(())
}
