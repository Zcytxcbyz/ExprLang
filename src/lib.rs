#![allow(non_snake_case)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod repl;
mod value;
pub mod error;

pub use evaluator::Evaluator;
pub use evaluator::{evaluate, evaluate_with_context, evaluate_with_env};
pub use repl::repl;
pub use value::Value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::f64::consts;

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

    #[test]
    fn test_arithmetic() {
        assert_eq!(eval_num("3 + 4 * 2"), 11.0);
        assert_eq!(eval_num("(3 + 4) * 2"), 14.0);
        assert_eq!(eval_num("10 / 2 - 3"), 2.0);
        assert_eq!(eval_num("2.5 * 4.8"), 12.0);
        assert_eq!(eval_num("1 + 2 + 3 * (4 + 5)"), 30.0);
        assert_eq!(eval_num("-3 + 5"), 2.0);
    }

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

    #[test]
    fn test_comparisons() {
        assert_eq!(eval_num("5 > 3"), 1.0);
        assert_eq!(eval_num("3 == 3"), 1.0);
        assert_eq!(eval_num("2 != 2"), 0.0);
        assert_eq!(eval_num("5 <= 5"), 1.0);
    }

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

    #[test]
    fn test_if_elif_else() {
        assert_eq!(eval_num("if 1 then 10 elif 2 then 20 else 30"), 10.0);
        assert_eq!(eval_num("if 0 then 10 elif 1 then 20 else 30"), 20.0);
        assert_eq!(eval_num("if 0 then 10 elif 0 then 20 else 30"), 30.0);
    }

    #[test]
    fn test_for_loop() {
        assert_eq!(eval_num("sum = 0; for i in 1..10 do sum = sum + i; sum"), 45.0);
        assert_eq!(eval_num("sum = 0; for i in 1..10 step 2 do sum = sum + i; sum"), 25.0);
        assert_eq!(eval_num("sum = 0; for i in 10..1 step -1 do sum = sum + i; sum"), 54.0);
    }

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

    #[test]
    fn test_slice() {
        let expr = "[1, 2, 3, 4, 5][1:4]";
        match evaluate(expr).unwrap() {
            Value::Array(a) => assert_eq!(a, vec![Value::Num(2.0), Value::Num(3.0), Value::Num(4.0)]),
            _ => panic!("Expected array"),
        }
    }

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

    #[test]
    fn test_functions() {
        let expr = "
            fn fact(n) = if n <= 1 then 1 else n * fact(n - 1);
            fact(5)
        ";
        assert_eq!(eval_num(expr), 120.0);
    }

    #[test]
    fn test_strings() {
        let expr = "\"hello, \" + \"world!\"";
        match evaluate(expr).unwrap() {
            Value::Str(s) => assert_eq!(s, "hello, world!"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_arrays() {
        let expr = "[1, 2, 3][1]";
        match evaluate(expr).unwrap() {
            Value::Num(n) => assert_eq!(n, 2.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_logic() {
        assert_eq!(eval_num("1 && 0"), 0.0);
        assert_eq!(eval_num("1 || 0"), 1.0);
        assert_eq!(eval_num("!0"), 1.0);
        assert_eq!(eval_num("!(3 > 2)"), 0.0);
    }

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
}
