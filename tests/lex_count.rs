use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// integration test: run `ruscom lex --count tests/data/sample1.cpp` and assert output is the expected token count
#[test]
fn lex_count_sample1() {
    // read sample just to confirm it's present
    let _ = fs::read_to_string("tests/data/sample1.cpp").expect("sample1.cpp missing");

    let mut cmd = Command::cargo_bin("ruscom").expect("binary not built");
    cmd.arg("lex").arg("--count").arg("tests/data/sample1.cpp");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("^\\d+\\n$").unwrap());
}
