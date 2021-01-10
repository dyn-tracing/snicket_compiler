use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");

    Ok(())
}
