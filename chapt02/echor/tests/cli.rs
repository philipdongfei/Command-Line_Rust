use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
//use std::fs::{self, File};
//use std::str;
//use std::fs::File;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())

}


#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
    /*
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    let args = &["Hello there"];
        cmd.args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
    */
    //expected.push('\n'); 
    //assert_eq!("Hello there", expected);
    /*
    let mut f = File::open(outfile);
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();
    let expected = str::from_utf8(&buffer).unwrap();
    */
    
    /*
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["Hello there"]).assert().success().stdout(expected);
    */
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
    /*
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
    */
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}


