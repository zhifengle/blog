use std::{env, error::Error};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

// mut stream
async fn process(mut stream: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await?;

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", "test");
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
