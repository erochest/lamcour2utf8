extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::predicate;
use std::path::Path;
use std::process::Command;

#[test]
fn test_recodes_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["dragonfly_"]).unwrap();
    let dragonfly = temp.child("dragonfly_");
    let dragonfly_utf8 = temp.child("dragonfly_.utf8");

    let command = Command::main_binary()
        .unwrap()
        .args(dragonfly.path().to_str())
        .unwrap();

    command.assert().success();

    let predicate_file = predicate::path::eq_file(Path::new("./tests/fixtures/dragonfly_.utf8"));
    dragonfly_utf8.assert(predicate_file);
}
