fn main() {
    let mut e = std::env::args();
    match e.next() {
        Some(s) => {
            println!("{}", s);
        }
        None => {
            println!("None");
        }
    }
}
