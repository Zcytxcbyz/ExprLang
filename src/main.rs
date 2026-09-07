use expr_lang::{Evaluator, Value, evaluate_with_context, repl};
use std::collections::HashMap;
use std::f64::consts;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        if file_path == "--help" || file_path == "-h" {
            println!("Usage:");
            println!("  cargo run                    Start interactive REPL");
            println!("  cargo run -- <script.txt>   Execute script file");
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
