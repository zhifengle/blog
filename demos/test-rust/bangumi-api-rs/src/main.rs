mod api_info;
mod auth;
mod constants;
mod http;

use std::{error::Error, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Application err: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let client = http::HttpClient::new(http::HttpClientOpts {
        ua: None,
        proxy_url: None,
    });
    let url = "https://bgm.tv/";
    let content = client.fetch_text(url)?;
    println!("{}", content);
    Ok(())
}
