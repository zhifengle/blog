mod arithmetic;
mod sedregex;

use nom::{bytes::complete::tag, IResult};

fn hello_parser(i: &str) -> IResult<&str, &str> {
    tag("hello")(i)
}
fn main() {
    println!("{:?}", hello_parser("hello"));
    println!("{:?}", hello_parser("hello world"));
    println!("{:?}", hello_parser("goodbye hello again"));
}
