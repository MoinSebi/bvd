use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

#[test]
/// Test of file does exist
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bvd")?;
    cmd
        .arg("--gfa")
        .arg("data/example_data/testGraph1.gfa")
        .arg("-o")
        .arg("./data/example_data/test1/test1");
    cmd.assert().stderr(predicate::str::contains("No file with such name"));

    Ok(())
}

#[test]
/// Check if a file does exist
fn file_does_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bvd")?;
    cmd
        .arg("--gfa")
        .arg("data/example_data/testGraph2.gfa")
        .arg("-o")
        .arg("./data/example_data/test1/test1");
    cmd.assert().success();
    Ok(())
}
