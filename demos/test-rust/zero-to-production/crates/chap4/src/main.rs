// 基于 actix

// https://docs.rs/actix-web/latest/actix_web/index.html
// https://actix.rs/docs/
// https://github.com/actix/examples

use chap4::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("ztp".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}

// ---------------------------------
// 5个 trait
// Responder, Handler, FromRequest
