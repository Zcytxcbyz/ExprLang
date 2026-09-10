//! ExprLang command-line interface.
//!
//! Provides a REPL (Read-Eval-Print Loop) for interactive use and
//! script execution from files.

use expr_lang::{Evaluator, VERSION, Value, evaluate_with_context, repl_with_max_steps};
use std::collections::HashMap;
use std::f64::consts;

/// Default maximum number of evaluation steps.
/// A negative value means unlimited.
const DEFAULT_MAX_STEPS: i64 = -1;

/// Entry point for the ExprLang CLI.
///
/// Usage:
///   exprlang [OPTIONS] [script.expr]
///
/// Options:
///   -s, --max-steps N    Maximum evaluation steps per input
///                        N < 0: unlimited (default)
///                        N > 0: limited to N steps
///   -V, --version        Show version and exit
///   -h, --help           Show help and exit
fn main() -> Result<(), String> {
    let mut max_steps: i64 = DEFAULT_MAX_STEPS;
    let mut file_path: Option<String> = None;
    let mut show_version = false;
    let mut show_help = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" | "-V" => show_version = true,
            "--help" | "-h" => show_help = true,
            "--max-steps" | "-s" => {
                let val = args
                    .next()
                    .ok_or_else(|| format!("Option '{}' requires a value", arg))?;
                max_steps = val.parse::<i64>().map_err(|_| {
                    format!(
                        "Invalid value for '{}': '{}' (expected an integer)",
                        arg, val
                    )
                })?;
            }
            _ if arg.starts_with("--max-steps=") => {
                let val = &arg["--max-steps=".len()..];
                max_steps = val.parse::<i64>().map_err(|_| {
                    format!(
                        "Invalid value for --max-steps: '{}' (expected an integer)",
                        val
                    )
                })?;
            }
            _ if arg.starts_with('-') && arg.len() > 1 => {
                return Err(format!("Unknown option: {}", arg));
            }
            _ => {
                if file_path.is_some() {
                    return Err(format!("Unexpected argument: {}", arg));
                }
                file_path = Some(arg);
            }
        }
    }

    if show_version {
        println!("ExprLang v{}", VERSION);
        return Ok(());
    }

    if show_help {
        print_help();
        return Ok(());
    }

    if let Some(path) = file_path {
        run_script(&path, max_steps)
    } else {
        repl_with_max_steps(max_steps)
    }
}

/// Executes a script file with the given step limit.
fn run_script(path: &str, max_steps: i64) -> Result<(), String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

    let mut env = HashMap::new();
    env.insert("pi".to_string(), Value::Num(consts::PI));
    env.insert("e".to_string(), Value::Num(consts::E));

    let mut evaluator = Evaluator::with_max_steps(max_steps);
    match evaluate_with_context(&content, &mut env, &mut evaluator) {
        Ok(val) => {
            println!("{}", val);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}

/// Prints the CLI help message.
fn print_help() {
    println!("ExprLang - A lightweight mathematical expression language");
    println!();
    println!("Usage:");
    println!("  exprlang [OPTIONS] [script.expr]");
    println!();
    println!("Options:");
    println!("  -s, --max-steps N    Maximum evaluation steps per input");
    println!("                       N < 0: unlimited (default)");
    println!("                       N > 0: limited to N steps");
    println!("  -V, --version        Show version and exit");
    println!("  -h, --help           Show this help and exit");
    println!();
    println!("Examples:");
    println!("  exprlang                                 Start interactive REPL");
    println!("  exprlang script.expr                     Execute script file");
    println!("  exprlang --max-steps 10000 s.expr        Execute with step limit");
    println!("  exprlang -s -1                           REPL with unlimited steps");
    println!();
    println!("With Cargo:");
    println!("  cargo run                                Start interactive REPL");
    println!("  cargo run -- script.expr                 Execute script file");
    println!("  cargo run -- --max-steps 10000 s.expr    Execute with step limit");
    println!("  cargo run -- --version                   Show version");
}
