// 基于 actix

// https://docs.rs/actix-web/latest/actix_web/index.html
// https://actix.rs/docs/
// https://github.com/actix/examples

use chap3::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}

// ---------------------------------
// 5个 trait
// Responder, Handler, FromRequest
