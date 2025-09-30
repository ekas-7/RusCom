use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn lex_count_all_samples() {
    let entries = fs::read_dir("tests/data").expect("tests/data directory missing");
    for entry in entries {
        let entry = entry.expect("read_dir entry");
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "cpp" {
                let p = path.to_string_lossy().into_owned();
                let mut cmd = Command::cargo_bin("ruscom").expect("binary not built");
                cmd.arg("lex").arg("--count").arg(&p);
                cmd.assert()
                    .success()
                    .stdout(predicate::str::is_match("^\\d+\\n$").unwrap())
                    .stderr(predicate::str::is_empty());
            }
        }
    }
}
