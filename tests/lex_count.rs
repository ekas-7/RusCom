use assert_cmd::Command;
use std::fs;

#[test]
fn lex_count_all_samples() {
    let entries = fs::read_dir("tests/data").expect("tests/data directory missing");
    for entry in entries {
        let entry = entry.expect("read_dir entry");
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "cpp" {
                let p = path.to_string_lossy().into_owned();
                // run binary to get the count
                let mut cmd = Command::cargo_bin("ruscom").expect("binary not built");
                let assert = cmd.arg("lex").arg("--count").arg(&p).assert().success();
                let out = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
                let bin_count: usize = out.trim().parse().expect("binary did not print a number");

                // run library lexer to collect tokens and log them
                let src = fs::read_to_string(&p).expect("read sample");
                let mut lex = ruscom::lexer::Lexer::new(&src);
                let mut tokens = Vec::new();
                while let Some(r) = lex.next() {
                    let t = r.expect("lex error");
                    if t == ruscom::lexer::token::Token::Eof { break; }
                    tokens.push(t);
                }
                eprintln!("{} tokens ({}): {:?}", &p, tokens.len(), tokens);
                assert_eq!(tokens.len(), bin_count, "count mismatch for {}", &p);
            }
        }
    }
}
