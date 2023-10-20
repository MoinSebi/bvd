
use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use std::fs;

#[test]
/// Testing pan-sv
/// Parameters.
///     --gfa
///     -o test1
///
fn main_solo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bvd")?;
    fs::create_dir_all("/home/svorbrugg/code/bvd/data/example_data/test1")?;
    cmd
        .arg("-g")
        .arg("data/example_data/testGraph2.gfa")
        .arg("-o")
        .arg("/home/svorbrugg/code/bvd/data/example_data/test1/dsadasda");

    cmd.assert().success();
    //let foo: String = fs::read_to_string("/home/svorbrugg/code/bvd/data/example_data/test1/test1.bed").unwrap();
    //assert_eq!(foo.contains("1	4	1"), true);

    //let path = "data/example_data/test1";
    //fs::remove_dir_all(path).unwrap();
    //fs::create_dir(path).unwrap();


    Ok(())
}




