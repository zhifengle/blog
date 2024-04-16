/// https://betterprogramming.pub/a-simple-guide-to-using-thiserror-crate-in-rust-eee6e442409b

// 使用 thiserror 后去掉了 Display, std::error::Error, From<xxx>

use std::{error::Error, fmt::Debug};

fn main() {
    println!("{:?}", make_request().unwrap_err());
}

fn make_request() -> Result<(), CustomError> {
    use CustomError::*;

    let key = std::fs::read_to_string("some-key-file").map_err(FileReadError)?;
    reqwest::blocking::get(format!("http:key/{}", key))?.error_for_status()?;
    std::fs::remove_file("some-key-file").map_err(FileDeleteError)?;
    Ok(())
}

#[derive(thiserror::Error)]
enum CustomError {
    #[error("failed to read the key file")]
    FileReadError(#[source] std::io::Error),

    #[error("failed to send the api request")]
    RequestError(#[from] reqwest::Error),

    #[error("failed to delete the key file")]
    FileDeleteError(#[source] std::io::Error),
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}