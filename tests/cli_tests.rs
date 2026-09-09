use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// 获取当前 crate 的二进制路径
fn get_binary_path() -> PathBuf {
    // 使用 env!("CARGO_BIN_EXE") 获取二进制路径
    // 需要将 crate 名改为 "expr_lang"（Cargo.toml 中的 name）
    // 由于当前包名是 expr_lang，二进制名也是 expr_lang
    let bin_name = if cfg!(windows) { "expr_lang.exe" } else { "expr_lang" };
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push(bin_name);
    if !path.exists() {
        // 如果不存在，尝试使用 cargo run 的路径
        let mut alt_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        alt_path.push("target");
        alt_path.push("debug");
        alt_path.push("expr_lang");
        if !alt_path.exists() {
            // 若仍不存在，直接使用 cargo run
            return PathBuf::from("cargo");
        }
        return alt_path;
    }
    path
}

/// 使用 cargo run 执行命令（确保编译）
fn run_with_cargo(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .args(args)
        .output()
        .expect("Failed to execute cargo run")
}

/// 直接运行二进制
fn run_binary(args: &[&str]) -> std::process::Output {
    let binary = get_binary_path();
    if binary.file_name().unwrap() == "cargo" {
        // fallback 到 cargo run
        run_with_cargo(args)
    } else {
        Command::new(&binary)
            .args(args)
            .output()
            .expect(&format!("Failed to execute {:?}", binary))
    }
}

// ===== 测试用例 =====

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
    assert!(stdout.contains("REPL"));
    assert!(stdout.contains("script"));
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
    // 创建临时脚本文件
    let script_content = "
        x = 10;
        x * 2 + 1
    ";
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("test_script.expr");
    fs::write(&script_path, script_content).expect("Failed to write test script");

    let output = run_binary(&[script_path.to_str().unwrap()]);
    let _ = fs::remove_file(&script_path); // 清理

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
    // 错误脚本应返回非零退出码
    let script_content = "1 / 0"; // 除零错误
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
    let script_content = "1 + + 2"; // 语法错误
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
    // 测试无参数时 REPL 启动（快速测试）
    // 由于 REPL 是交互式的，我们只测试它能正常启动并接受 exit 命令
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
        stdin.write_all(b"exit\n").expect("Failed to write to stdin");
        stdin.flush().expect("Failed to flush stdin");
    });

    let output = child.wait_with_output().expect("Failed to wait for REPL");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // REPL 启动时应显示版本信息
    assert!(stdout.contains("ExprLang v"));
    assert!(stdout.contains("Supported:"));
}
