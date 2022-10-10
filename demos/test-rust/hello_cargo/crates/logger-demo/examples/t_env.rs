use env_logger::Builder;
use log::info;

// https://github.com/env-logger-rs/env_logger/

fn main() {
    use std::env;

    let key = "RUST_LOG";
    env::set_var(key, "info");
    assert_eq!(env::var(key), Ok("info".to_string()));

    env_logger::init();

    info!("starting up");

    // ...
}
