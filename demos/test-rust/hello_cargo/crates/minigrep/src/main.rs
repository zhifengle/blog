use std::{env, process};

use minigrep::{run, Config};

// cargo run --package minigrep --bin minigrep the poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parsing arguments err: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
