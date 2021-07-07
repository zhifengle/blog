mod http;
mod qiandao;

use crate::http::{HttpClient, HttpClientOpts};
use clap::App;
use std::{error::Error, process};

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg("-c, --cookie=[FILE] 'Sets a cookie file'")
        .get_matches();
    let cookie_file = matches.value_of("cookie").unwrap();
    println!("Value for config: {}", cookie_file);

    // if let Err(e) = run() {
    //     eprintln!("Application err: {}", e);

    //     process::exit(1);
    // }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn Error>> {
    let url = "https://bgm.tv/";
    let client = HttpClient::new(HttpClientOpts {
        ua: None,
        proxy_url: Some("http://127.0.0.1:10809".to_string()),
    });
    let content = client.fetch_text(url).await?;
    assert!(content.contains("动漫"));
    println!("{}", content.contains("动漫"));
    Ok(())
}
