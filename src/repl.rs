//! Interactive REPL (Read-Eval-Print Loop) for ExprLang.
//!
//! Allows users to interactively enter expressions and see results
//! in real time.

use crate::{Evaluator, VERSION, Value, evaluate_with_context};
use std::collections::HashMap;
use std::f64::consts;
use std::io::{self, Write};

/// Runs the interactive REPL with no step limit.
pub fn repl() -> Result<(), String> {
    repl_with_max_steps(-1)
}

/// Runs the interactive REPL with the given step limit.
///
/// A negative `max_steps` (or zero) means unlimited.
/// The step counter is reset before each input, so every user
/// submission gets a fresh budget.
pub fn repl_with_max_steps(max_steps: i64) -> Result<(), String> {
    let mut env = HashMap::new();
    env.insert("pi".to_string(), Value::Num(consts::PI));
    env.insert("e".to_string(), Value::Num(consts::E));

    let mut evaluator = Evaluator::with_max_steps(max_steps);

    println!("ExprLang v{} (Rust Math Expression Language)", VERSION);
    println!("Supported: arithmetic, comparisons, logic (&&, ||, !),");
    println!("strings, arrays, indexing, slicing, functions, loops, conditions.");
    if max_steps > 0 {
        println!("Step limit: {}", max_steps);
    } else {
        println!("Step limit: unlimited");
    }
    println!("Type 'exit' or 'quit' to exit.");
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

        // Each input gets its own step budget.
        evaluator.reset_steps();

        match evaluate_with_context(input, &mut env, &mut evaluator) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
