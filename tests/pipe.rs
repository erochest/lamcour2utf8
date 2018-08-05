extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_recodes_from_stdin() {
    let command = Command::main_binary()
        .unwrap()
        .with_stdin()
        .buffer("darning needle,NY3A,401,,N, , ,dµº´n±¿n$ni½ºdì,MS,5\n")
        .unwrap();
    command.assert().success().stdout(
        "darning needle,NY3A,401,,N, , ,d\u{0299}\u{0251}\u{0299}\u{00b7}\u{0299}\u{025a}n\u{0299}\u{026a}\u{0299}\u{02d7}n$ni\u{0299}\u{02c5}\u{0299}\u{00b7}d\u{02b9}\u{0327},MS,5\n",
    );
}
