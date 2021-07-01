mod api_info;
mod auth;
mod http;

use crate::http::HttpClient;
use std::{error::Error, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Application err: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let client = HttpClient::new();
    let url = "https://bgm.tv/";
    let content = client.fetch_text(url)?;
    println!("{}", content);
    Ok(())
}
