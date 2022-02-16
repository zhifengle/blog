use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &str = "arc-7z";

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn t_archive_folder() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("C:\\Devel")
        .arg("-o C:\\")
        .assert()
        .code(0);
    Ok(())
}
