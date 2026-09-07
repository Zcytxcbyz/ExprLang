use ExprLang::{evaluate, evaluate_with_context, Value, Evaluator};
use std::collections::HashMap;

#[test]
fn test_factorial_recursive() {
    let script = "
        fn fact(n) = if n <= 1 then 1 else n * fact(n - 1);
        fact(5)
    ";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 120.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_loop_sum() {
    let script = "
        sum = 0;
        i = 1;
        while i <= 10 do (
            sum = sum + i;
            i = i + 1
        );
        sum
    ";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 55.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_array_index() {
    let script = "[1, 2, 3][1]";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 2.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_string_concat() {
    let script = "\"Hello, \" + \"world!\"";
    match evaluate(script).unwrap() {
        Value::Str(s) => assert_eq!(s, "Hello, world!"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_condition() {
    let script = "if 3 > 2 then 100 else 200";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 100.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_persistent_evaluator() {
    let mut env = HashMap::new();
    let mut evaluator = Evaluator::new();
    let _ = evaluate_with_context("fn square(x) = x * x;", &mut env, &mut evaluator).unwrap();
    let result = evaluate_with_context("square(5)", &mut env, &mut evaluator).unwrap();
    assert_eq!(result, Value::Num(25.0));
}

#[test]
fn test_builtins_extra_integration() {
    let script = "max(3, 5) + min(10, 2)";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 7.0), // 5 + 2
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_nested_array() {
    let script = "[[1,2],[3,4]][1][0]";
    match evaluate(script).unwrap() {
        Value::Num(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number"),
    }
}
