pub mod http;
pub mod qiandao;
pub mod storage;
use std::error::Error;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application err: {}", e);

        std::process::exit(1);
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
