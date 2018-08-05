extern crate assert_cmd;
extern crate assert_fs;
extern crate dir_diff;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn test_recodees_directory_tree() {
    let temp = assert_fs::TempDir::new().unwrap();

    temp.copy_from("./tests/fixtures", &["dragonfly_"]).unwrap();
    temp.copy_from("./tests/fixtures", &["one/eight_"]).unwrap();
    temp.copy_from("./tests/fixtures", &["one/two/Virginia_"])
        .unwrap();

    let command = Command::main_binary()
        .unwrap()
        .arg(temp.path().to_str().unwrap())
        .unwrap();

    command.assert().success();

    Command::new("tree")
        .arg(temp.path().to_str().unwrap())
        .spawn()
        .unwrap();

    assert!(!dir_diff::is_different("./tests/fixtures", temp.path()).unwrap());
}
