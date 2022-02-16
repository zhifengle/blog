use head_rs::{get_matches, run, Config};

fn main() {
    let matches = get_matches();
    if let Err(e) = Config::from_matches(matches).and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
