//! ExprLang: A lightweight mathematical expression language.
//!
//! This crate provides a complete interpreter for ExprLang, supporting
//! arithmetic, variables, functions, loops, conditionals, arrays, and more.
//!
//! # Examples
//!
//! ```
//! use expr_lang::evaluate;
//!
//! let result = evaluate("3 + 4 * 2").unwrap();
//! assert_eq!(result.to_string(), "11");
//! ```

#![allow(non_snake_case)]

mod ast;
pub mod error;
mod evaluator;
mod lexer;
mod parser;
mod repl;
mod value;

pub use evaluator::Evaluator;
pub use evaluator::{evaluate, evaluate_with_context, evaluate_with_env};
pub use repl::repl;
pub use value::Value;

/// The current version of ExprLang, read from Cargo.toml at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::f64::consts;

    /// Helper: evaluate an expression and extract a numeric result.
    fn eval_num(expr: &str) -> f64 {
        match evaluate(expr).unwrap() {
            Value::Num(n) => n,
            _ => panic!("Expected number"),
        }
    }

    #[allow(dead_code)]
    fn eval_str(expr: &str) -> String {
        match evaluate(expr).unwrap() {
            Value::Str(s) => s,
            _ => panic!("Expected string"),
        }
    }

    // ===== Basic arithmetic =====

    #[test]
    fn test_arithmetic() {
        assert_eq!(eval_num("3 + 4 * 2"), 11.0);
        assert_eq!(eval_num("(3 + 4) * 2"), 14.0);
        assert_eq!(eval_num("10 / 2 - 3"), 2.0);
        assert_eq!(eval_num("2.5 * 4.8"), 12.0);
        assert_eq!(eval_num("1 + 2 + 3 * (4 + 5)"), 30.0);
        assert_eq!(eval_num("-3 + 5"), 2.0);
    }

    // ===== Built-in functions (v0.1.0) =====

    #[test]
    fn test_builtins_old() {
        assert_eq!(eval_num("sin(pi/2)"), 1.0);
        assert_eq!(eval_num("cos(0)"), 1.0);
        assert_eq!(eval_num("sqrt(16)"), 4.0);
        assert_eq!(eval_num("pow(2, 3)"), 8.0);
        assert_eq!(eval_num("abs(-5)"), 5.0);
        assert_eq!(eval_num("floor(3.7)"), 3.0);
        assert_eq!(eval_num("ceil(3.2)"), 4.0);
        assert_eq!(eval_num("round(3.5)"), 4.0);
    }

    // ===== New built-in functions (v0.3.0) =====

    #[test]
    fn test_new_builtins() {
        assert_eq!(eval_num("atan2(1, 1) * 4"), consts::PI);
        assert_eq!(eval_num("log2(8)"), 3.0);
        assert_eq!(eval_num("log(100, 10)"), 2.0);
        assert_eq!(eval_num("hypot(3, 4)"), 5.0);
        assert_eq!(eval_num("factorial(5)"), 120.0);
        assert_eq!(eval_num("sign(-10)"), -1.0);
        assert_eq!(eval_num("sign(0)"), 0.0);
        assert_eq!(eval_num("is_even(4)"), 1.0);
        assert_eq!(eval_num("is_odd(5)"), 1.0);
        assert_eq!(eval_num("deg(pi)"), 180.0);
        assert_eq!(eval_num("rad(180)"), consts::PI);
    }

    // ===== Comparisons =====

    #[test]
    fn test_comparisons() {
        assert_eq!(eval_num("5 > 3"), 1.0);
        assert_eq!(eval_num("3 == 3"), 1.0);
        assert_eq!(eval_num("2 != 2"), 0.0);
        assert_eq!(eval_num("5 <= 5"), 1.0);
    }

    // ===== Variables and sequences =====

    #[test]
    fn test_variables_and_sequence() {
        let mut env = HashMap::new();
        let expr = "let x = 10; x * 2 + 1";
        let val = evaluate_with_env(expr, &mut env).unwrap();
        assert_eq!(val, Value::Num(21.0));
        let expr2 = "x = x + 1; x";
        let val2 = evaluate_with_env(expr2, &mut env).unwrap();
        assert_eq!(val2, Value::Num(11.0));
    }

    // ===== if-elif-else =====

    #[test]
    fn test_if_elif_else() {
        assert_eq!(eval_num("if 1 then 10 elif 2 then 20 else 30"), 10.0);
        assert_eq!(eval_num("if 0 then 10 elif 1 then 20 else 30"), 20.0);
        assert_eq!(eval_num("if 0 then 10 elif 0 then 20 else 30"), 30.0);
    }

    // ===== for loop =====

    #[test]
    fn test_for_loop() {
        assert_eq!(
            eval_num("sum = 0; for i in 1..10 do sum = sum + i; sum"),
            45.0
        );
        assert_eq!(
            eval_num("sum = 0; for i in 1..10 step 2 do sum = sum + i; sum"),
            25.0
        );
        assert_eq!(
            eval_num("sum = 0; for i in 10..1 step -1 do sum = sum + i; sum"),
            54.0
        );
    }

    // ===== break and continue =====

    #[test]
    fn test_break_continue() {
        let expr = "
            sum = 0;
            i = 1;
            while i <= 10 do (
                if i == 5 then break;
                sum = sum + i;
                i = i + 1
            );
            sum
        ";
        assert_eq!(eval_num(expr), 10.0);
    }

    // ===== slicing =====

    #[test]
    fn test_slice() {
        let expr = "[1, 2, 3, 4, 5][1:4]";
        match evaluate(expr).unwrap() {
            Value::Array(a) => {
                assert_eq!(a, vec![Value::Num(2.0), Value::Num(3.0), Value::Num(4.0)])
            }
            _ => panic!("Expected array"),
        }
    }

    // ===== while loop =====

    #[test]
    fn test_loop() {
        let expr = "
            sum = 0;
            i = 1;
            while i <= 10 do (
                sum = sum + i;
                i = i + 1
            );
            sum
        ";
        assert_eq!(eval_num(expr), 55.0);
    }

    // ===== Functions and recursion =====

    #[test]
    fn test_functions() {
        let expr = "
            fn fact(n) = if n <= 1 then 1 else n * fact(n - 1);
            fact(5)
        ";
        assert_eq!(eval_num(expr), 120.0);
    }

    // ===== Strings =====

    #[test]
    fn test_strings() {
        let expr = "\"hello, \" + \"world!\"";
        match evaluate(expr).unwrap() {
            Value::Str(s) => assert_eq!(s, "hello, world!"),
            _ => panic!("Expected string"),
        }
    }

    // ===== Arrays =====

    #[test]
    fn test_arrays() {
        let expr = "[1, 2, 3][1]";
        match evaluate(expr).unwrap() {
            Value::Num(n) => assert_eq!(n, 2.0),
            _ => panic!("Expected number"),
        }
    }

    // ===== Logic =====

    #[test]
    fn test_logic() {
        assert_eq!(eval_num("1 && 0"), 0.0);
        assert_eq!(eval_num("1 || 0"), 1.0);
        assert_eq!(eval_num("!0"), 1.0);
        assert_eq!(eval_num("!(3 > 2)"), 0.0);
    }

    // ===== Comments =====

    #[test]
    fn test_comments() {
        assert_eq!(eval_num("3 + 4  # comment\n"), 7.0);
        assert_eq!(eval_num("# only comment\n 5"), 5.0);
    }

    #[test]
    fn test_multiline_comment() {
        assert_eq!(eval_num("3 /* comment */ + 4"), 7.0);
        assert_eq!(eval_num("3 /* outer /* inner */ outer */ + 4"), 7.0);
    }

    // ===== String escapes =====

    #[test]
    fn test_string_escapes() {
        let expr = "\"hello\\nworld\"";
        match evaluate(expr).unwrap() {
            Value::Str(s) => assert_eq!(s, "hello\nworld"),
            _ => panic!("Expected string"),
        }
        let expr2 = "\"tab\\tbetween\"";
        match evaluate(expr2).unwrap() {
            Value::Str(s) => assert_eq!(s, "tab\tbetween"),
            _ => panic!("Expected string"),
        }
        let expr3 = "\"quote: \\\"inside\\\"\"";
        match evaluate(expr3).unwrap() {
            Value::Str(s) => assert_eq!(s, "quote: \"inside\""),
            _ => panic!("Expected string"),
        }
    }

    // ===== Error handling =====

    #[test]
    fn test_error_handling() {
        assert!(evaluate("1 / 0").is_err());
        assert!(evaluate("x + 1").is_err());
        assert!(evaluate("1 + \"a\"").is_err());
        assert!(evaluate("[1,2,3][5]").is_err());
        assert!(evaluate("sin(1, 2)").is_err());
        assert!(evaluate("3 = 4").is_err());
        assert!(evaluate("foo(1)").is_err());
        assert!(evaluate("\"a\" / 2").is_err());
        assert!(evaluate("(1 + 2").is_err());
        assert!(evaluate("1 + + 2").is_err());
    }

    // ===== Function scope =====

    #[test]
    fn test_function_scope() {
        let mut env = HashMap::new();
        let mut evaluator = Evaluator::new();
        let _ = evaluate_with_context("x = 10; fn f() = x;", &mut env, &mut evaluator).unwrap();
        let _ = evaluate_with_context("x = 20;", &mut env, &mut evaluator).unwrap();
        let result = evaluate_with_context("f()", &mut env, &mut evaluator).unwrap();
        assert_eq!(result, Value::Num(20.0));
        assert_eq!(env.get("x").unwrap(), &Value::Num(20.0));
        let _ = evaluate_with_context("fn g() = (y = 99; y);", &mut env, &mut evaluator).unwrap();
        let _ = evaluate_with_context("g()", &mut env, &mut evaluator).unwrap();
        assert!(!env.contains_key("y"));
    }

    // ===== Array edge cases =====

    #[test]
    fn test_array_edge_cases() {
        match evaluate("[]").unwrap() {
            Value::Array(a) => assert_eq!(a.len(), 0),
            _ => panic!("Expected array"),
        }
        match evaluate("[[1,2],[3,4]][0][1]").unwrap() {
            Value::Num(n) => assert_eq!(n, 2.0),
            _ => panic!("Expected number"),
        }
        assert_eq!(eval_num("[1+2, 3*4][1]"), 12.0);
    }

    // ===== String edge cases =====

    #[test]
    fn test_string_edge_cases() {
        match evaluate("\"\"").unwrap() {
            Value::Str(s) => assert_eq!(s, ""),
            _ => panic!("Expected string"),
        }
        let expr = "\"Hello\\n\" + \"World\"";
        match evaluate(expr).unwrap() {
            Value::Str(s) => assert_eq!(s, "Hello\nWorld"),
            _ => panic!("Expected string"),
        }
    }

    // ===== Additional tests (v0.3.0) =====

    #[test]
    fn test_for_step_zero() {
        assert!(evaluate("for i in 1..10 step 0 do 1").is_err());
    }

    #[test]
    fn test_for_break() {
        let expr = "
            sum = 0;
            for i in 1..10 do (
                if i > 5 then break else 0;
                sum = sum + i
            );
            sum
        ";
        assert_eq!(eval_num(expr), 15.0);
    }

    #[test]
    fn test_for_continue() {
        let expr = "
            sum = 0;
            for i in 1..10 do (
                if is_even(i) then continue else 0;
                sum = sum + i
            );
            sum
        ";
        assert_eq!(eval_num(expr), 25.0);
    }

    #[test]
    fn test_slice_out_of_bounds() {
        assert!(evaluate("[1,2,3][3:5]").is_err());
    }

    #[test]
    fn test_slice_start_end_none() {
        let expr = "[1,2,3][0:3]";
        match evaluate(expr).unwrap() {
            Value::Array(a) => {
                assert_eq!(a, vec![Value::Num(1.0), Value::Num(2.0), Value::Num(3.0)])
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_builtin_type_errors() {
        assert!(evaluate("log(\"a\", 2)").is_err());
        assert!(evaluate("hypot(\"a\", 3)").is_err());
        assert!(evaluate("atan2(1, \"b\")").is_err());
        assert!(evaluate("factorial(\"5\")").is_err());
        assert!(evaluate("sign([])").is_err());
        assert!(evaluate("is_even(3.5)").is_err());
        assert!(evaluate("deg(\"pi\")").is_err());
    }

    #[test]
    fn test_value_display() {
        let empty = Value::Array(vec![]);
        assert_eq!(empty.to_string(), "[]");
        let single = Value::Array(vec![Value::Num(42.0)]);
        assert_eq!(single.to_string(), "[42]");
        let nested = Value::Array(vec![
            Value::Array(vec![Value::Num(1.0), Value::Num(2.0)]),
            Value::Array(vec![Value::Num(3.0), Value::Num(4.0)]),
        ]);
        assert_eq!(nested.to_string(), "[[1, 2], [3, 4]]");
        let mixed = Value::Array(vec![Value::Num(1.0), Value::Str("hello".to_string())]);
        assert_eq!(mixed.to_string(), "[1, \"hello\"]");
    }

    #[test]
    fn test_for_negative_step() {
        let expr = "
            sum = 0;
            for i in 10..1 step -1 do sum = sum + i;
            sum
        ";
        assert_eq!(eval_num(expr), 54.0);
    }

    #[test]
    fn test_for_non_numeric_bounds() {
        assert!(evaluate("for i in \"a\"..10 do 1").is_err());
        assert!(evaluate("for i in 1..\"b\" do 1").is_err());
    }

    #[test]
    fn test_undefined_character() {
        let result = evaluate("3 + {");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected character: {"));
        assert!(evaluate("@").is_err());
        assert!(evaluate("3 $ 4").is_err());
    }

    #[test]
    fn test_lone_dot_is_error() {
        // A lone '.' must produce an error, not hang or panic.
        assert!(evaluate(".").is_err());
        assert!(evaluate(". + 1").is_err());
        assert!(evaluate("3 + .").is_err());
        assert!(evaluate("[1,2,3][.]").is_err());
        assert!(evaluate(".a").is_err());
    }

    #[test]
    fn test_leading_dot_number() {
        // '.5' should still parse as 0.5
        assert_eq!(eval_num(".5"), 0.5);
        assert_eq!(eval_num(".25 * 4"), 1.0);
        assert_eq!(eval_num("[.5, .25][0]"), 0.5);
    }
}
