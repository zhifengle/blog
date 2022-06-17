struct Parser {
    pos: usize,
    input: String,
}

fn main() {
    let pos: usize = 0;

    let input = "input".to_string();
    let mut p = Parser { pos: 0, input };
    println!("{}", input[pos..].chars().next().unwrap())
}
