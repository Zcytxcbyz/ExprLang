use std::collections::HashMap;
use std::io::{self, Write};
use std::f64::consts;
use crate::{evaluate_with_context, Evaluator, Value};

pub fn repl() -> Result<(), String> {
    let mut env = HashMap::new();
    env.insert("pi".to_string(), Value::Num(consts::PI));
    env.insert("e".to_string(), Value::Num(consts::E));

    let mut evaluator = Evaluator::new();

    println!("ExprLang v0.1.0 (Rust Math Expression Language)");
    println!("Supported: arithmetic, comparisons, logic (&&, ||, !),");
    println!("strings, arrays, indexing, functions, loops, conditions.");
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        match evaluate_with_context(input, &mut env, &mut evaluator) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
