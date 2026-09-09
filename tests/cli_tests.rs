//! Integration tests for the ExprLang CLI.
//!
//! Tests command-line argument handling, script execution,
//! version output, and error handling.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Get the path to the built binary.
///
/// This attempts to find the binary in the target/debug directory,
/// with fallback to `cargo run` if not found.
fn get_binary_path() -> PathBuf {
    let bin_name = if cfg!(windows) {
        "expr_lang.exe"
    } else {
        "expr_lang"
    };
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push(bin_name);
    if !path.exists() {
        let mut alt_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        alt_path.push("target");
        alt_path.push("debug");
        alt_path.push("expr_lang");
        if !alt_path.exists() {
            return PathBuf::from("cargo");
        }
        return alt_path;
    }
    path
}

/// Run a command using `cargo run`.
fn run_with_cargo(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .args(args)
        .output()
        .expect("Failed to execute cargo run")
}

/// Run the binary with the given arguments.
fn run_binary(args: &[&str]) -> std::process::Output {
    let binary = get_binary_path();
    if binary.file_name().unwrap() == "cargo" {
        run_with_cargo(args)
    } else {
        Command::new(&binary)
            .args(args)
            .output()
            .expect(&format!("Failed to execute {:?}", binary))
    }
}

#[test]
fn test_cli_version() {
    let output = run_binary(&["--version"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ExprLang v"));
    assert!(stdout.contains("0.3.0"));
}

#[test]
fn test_cli_version_short() {
    let output = run_binary(&["-V"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ExprLang v"));
    assert!(stdout.contains("0.3.0"));
}

#[test]
fn test_cli_help() {
    let output = run_binary(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Start interactive REPL"));
    assert!(stdout.contains("Execute script file"));
    assert!(stdout.contains("Show version"));
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
