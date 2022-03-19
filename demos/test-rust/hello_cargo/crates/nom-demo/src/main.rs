mod arithmetic;
mod sedregex;

use nom::{bytes::complete::tag, IResult};

// https://privaterookie.gitee.io/nom-cheatsheet/
// https://github.com/fucking-translation/tutorial/blob/main/Rust/nom/%E9%80%89%E6%8B%A9nom%E7%BB%84%E5%90%88%E5%99%A8.md

fn hello_parser(i: &str) -> IResult<&str, &str> {
    tag("hello")(i)
}
fn main() {
    println!("{:?}", hello_parser("hello"));
    println!("{:?}", hello_parser("hello world"));
    println!("{:?}", hello_parser("goodbye hello again"));
}
