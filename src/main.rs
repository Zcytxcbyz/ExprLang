//! ExprLang command-line interface.
//!
//! Provides a REPL (Read-Eval-Print Loop) for interactive use and
//! script execution from files.

use expr_lang::{Evaluator, Value, evaluate_with_context, repl, VERSION};
use std::collections::HashMap;
use std::f64::consts;

/// Entry point for the ExprLang CLI.
///
/// - With no arguments: starts the REPL.
/// - With `--version` or `-V`: prints the version and exits.
/// - With a file path: executes the script file and prints the result.
/// - With `--help` or `-h`: prints usage information.
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    // Handle version flag
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("ExprLang v{}", VERSION);
        return Ok(());
    }

    // Handle file execution or help
    if args.len() > 1 {
        let file_path = &args[1];
        if file_path == "--help" || file_path == "-h" {
            println!("Usage:");
            println!("  cargo run                    Start interactive REPL");
            println!("  cargo run -- <script.txt>   Execute script file");
            println!("  cargo run -- --version      Show version");
            return Ok(());
        }

        let content = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

        let mut env = HashMap::new();
        env.insert("pi".to_string(), Value::Num(consts::PI));
        env.insert("e".to_string(), Value::Num(consts::E));

        let mut evaluator = Evaluator::new();
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
    } else {
        repl()
    }
}
