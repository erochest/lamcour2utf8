extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt, OutputOkExt};
use assert_fs::prelude::*;
use predicates::prelude::predicate;
use std::path::Path;
use std::process::Command;

#[test]
fn test_recodes_file_list() {
    let temp = assert_fs::TempDir::new().unwrap();

    temp.copy_from("./tests/fixtures", &["dragonfly_"]).unwrap();
    temp.copy_from("./tests/fixtures", &["one/eight_"]).unwrap();

    let dragonfly = temp.child("dragonfly_");
    let dragonfly_utf8 = temp.child("dragonfly_.utf8");
    let eight = temp.child("one/eight_");
    let eight_utf8 = temp.child("one/eight_.utf8");

    let command = Command::main_binary()
        .unwrap()
        .arg(dragonfly.path().to_str().unwrap())
        .arg(eight.path().to_str().unwrap())
        .unwrap();

    command.assert().success();

    let dragonfly_predicate =
        predicate::path::eq_file(Path::new("./tests/fixtures/dragonfly_.utf8"));
    dragonfly_utf8.assert(dragonfly_predicate);

    let eight_predicate = predicate::path::eq_file(Path::new("./tests/fixtures/one/eight_.utf8"));
    eight_utf8.assert(eight_predicate);
}
