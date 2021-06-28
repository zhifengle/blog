mod client;

use std::{error::Error, process};

use client::fetch_text;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application err: {}", e);

        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let url = "https://bgm.tv/";
    let content = fetch_text(url)?;
    println!("{}", content);
    Ok(())
}
