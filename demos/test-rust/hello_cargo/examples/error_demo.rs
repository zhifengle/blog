// https://www.philipdaniels.com/blog/2019/defining-rust-error-types/
// https://blog.burntsushi.net/rust-error-handling/

use std::{fmt, io};

// step1 定义错误
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Regular(ErrorKind),
    Custom(String),
}

// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    NotAuthorized,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match self {
            ErrorKind::NotFound => "not found",
            ErrorKind::NotAuthorized => "not authorized",
        }
    }
}

// -----------------------------------------------------------

// step2 实现 Error 和 Display trait

// description 已经废弃
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(err) => err.fmt(f),
            MyError::Regular(err) => write!(f, "A regular error occurred {:?}", err),
            MyError::Custom(err) => write!(f, "A custom error occurred {:?}", err),
        }
    }
}

// ---------------------------------------------------------------

// step3 实现 From
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

// 使用
fn test_func() -> Result<usize, MyError> {
    let _f = std::fs::File::open("some path")?;

    Err(MyError::Regular(ErrorKind::NotAuthorized))
}

fn main() {
    assert!(test_func().is_err());
    println!("{:?}", test_func().unwrap_err());
}
