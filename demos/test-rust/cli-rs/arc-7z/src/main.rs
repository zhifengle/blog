use arc_7z::{get_matches, run, Config};

fn main() {
    let matches = get_matches();

    if let Err(e) = Config::new(&matches).and_then(|c| run(&c)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
