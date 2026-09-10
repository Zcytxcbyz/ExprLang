//! Integration tests for the ExprLang CLI.
//!
//! Tests command-line argument handling, script execution,
//! version output, and error handling.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Returns the path to the `expr_lang` binary built by Cargo.
///
/// Cargo injects `CARGO_BIN_EXE_expr_lang` into the process environment
/// when running integration tests, and it always points to the freshly
/// built binary for the active target directory (including
/// `target/llvm-cov-target` when running under `cargo llvm-cov`).
fn binary_path() -> PathBuf {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_expr_lang") {
        return PathBuf::from(p);
    }
    // Fallback for unusual build setups that don't provide the env var.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push(if cfg!(windows) {
        "expr_lang.exe"
    } else {
        "expr_lang"
    });
    path
}

/// Run the binary with the given arguments.
fn run_binary(args: &[&str]) -> std::process::Output {
    Command::new(binary_path())
        .args(args)
        .output()
        .expect("Failed to execute expr_lang binary")
}

#[test]
fn test_cli_version() {
    let output = run_binary(&["--version"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ExprLang v"));
    let version = env!("CARGO_PKG_VERSION");
    assert!(stdout.contains(version));
}

#[test]
fn test_cli_version_short() {
    let output = run_binary(&["-V"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ExprLang v"));
    let version = env!("CARGO_PKG_VERSION");
    assert!(stdout.contains(version));
}

#[test]
fn test_cli_help() {
    let output = run_binary(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("version"));
}

#[test]
fn test_cli_help_short() {
    let output = run_binary(&["-h"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_cli_script_execution() {
    // Create a temporary script file
    let script_content = "
        x = 10;
        x * 2 + 1
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("21"));
}

#[test]
fn test_cli_script_with_function() {
    let script_content = "
        fn square(x) = x * x;
        square(5)
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script2.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("25"));
}

#[test]
fn test_cli_script_with_loop() {
    let script_content = "
        sum = 0;
        i = 1;
        while i <= 10 do (
            sum = sum + i;
            i = i + 1
        );
        sum
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script3.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("55"));
}

#[test]
fn test_cli_script_error() {
    // Error script should return a non-zero exit code
    let script_content = "1 / 0"; // Division by zero
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script_error.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"));
}

#[test]
fn test_cli_script_syntax_error() {
    let script_content = "1 + + 2"; // Syntax error
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script_syntax.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"));
}

#[test]
fn test_cli_script_not_found() {
    let output = run_binary(&["non_existent_file.expr"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read file"));
}

#[test]
fn test_cli_no_args_repl() {
    // Test REPL starts with no arguments
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn REPL");

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(b"exit\n")
            .expect("Failed to write to stdin");
        stdin.flush().expect("Failed to flush stdin");
    });

    let output = child.wait_with_output().expect("Failed to wait for REPL");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // REPL should display version info
    assert!(stdout.contains("ExprLang v"));
    assert!(stdout.contains("Supported:"));
}

#[test]
fn test_cli_max_steps_exceeded() {
    let script_content = "
        sum = 0;
        for i in 1..100 do sum = sum + i;
        sum
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_max_steps_low.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&["--max-steps", "10", script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Step limit exceeded"));
}

#[test]
fn test_cli_max_steps_within_bounds() {
    let script_content = "1 + 2 + 3";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_max_steps_high.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&["--max-steps", "10000", script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("6"));
}

#[test]
fn test_cli_max_steps_short_flag() {
    let script_content = "1 + 2";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_max_steps_short.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&["-s", "10000", script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
}

#[test]
fn test_cli_max_steps_unlimited_negative() {
    let script_content = "
        sum = 0;
        for i in 1..100 do sum = sum + i;
        sum
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_max_steps_unlimited.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&["--max-steps", "-1", script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("4950"));
}

#[test]
fn test_cli_max_steps_inline_equals() {
    let script_content = "1 + 2";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_max_steps_inline.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&["--max-steps=10000", script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path);

    assert!(output.status.success());
}

#[test]
fn test_cli_max_steps_invalid_value() {
    let output = run_binary(&["--max-steps", "abc"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid value"));
}

#[test]
fn test_cli_max_steps_missing_value() {
    let output = run_binary(&["--max-steps"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires a value"));
}

#[test]
fn test_cli_help_mentions_max_steps() {
    let output = run_binary(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--max-steps"));
    assert!(stdout.contains("unlimited"));
}
