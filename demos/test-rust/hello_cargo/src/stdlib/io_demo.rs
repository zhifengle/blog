#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
// 对于 file.read_to_string 是必须的
use std::io::Read;
use std::{error::Error as StdError, fs::OpenOptions};

// ref: scraping  storage.rs
#[test]
fn t_read() {
    let _s = std::fs::read("Cargo.toml").unwrap();
}

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
