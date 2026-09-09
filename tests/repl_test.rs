//! Tests for the REPL functionality.
//!
//! Verifies that the REPL starts and exits correctly.

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_repl_exit() {
    // Spawn the REPL and send "exit" command
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
