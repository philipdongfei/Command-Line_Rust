#[test]
fn works() {
    assert!(true);
}

use assert_cmd::Command; //use std::process::Command;

#[test]
fn runs() {
    /*
    let mut cmd = Command::new("hello"); // "ls",...
    let res = cmd.output();
    assert!(res.is_ok());
    */
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();

}
