use std::env;
use std::fs;

// RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
// #![allow(dead_code)]  ; crate level ??

#[allow(dead_code)]
pub fn mingrep_main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("args = {:?}", args);
    println!("query = {}; filename = {}", query, filename);

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// cargo test minigrep_tests -- --show-output
// 单独测试
#[cfg(test)]
mod minigrep_tests {
    use super::*;
    #[test]
    fn read_args_test() {
        mingrep_main();
    }
}
