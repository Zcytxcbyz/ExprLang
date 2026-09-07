// tests/repl_test.rs
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_repl_exit() {
    let mut child = Command::new("cargo")
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"exit\n").unwrap();
    stdin.flush().unwrap();

    let status = child.wait().unwrap();
    assert!(status.success());
}
