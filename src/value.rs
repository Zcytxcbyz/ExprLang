//! Defines the core value types used throughout the interpreter.
//!
//! Values are the runtime representation of data in ExprLang, supporting
//! numbers, strings, and arrays.

use std::fmt;

/// Represents a runtime value in ExprLang.
///
/// Values are dynamically typed and can be one of three variants:
/// - `Num`: a 64-bit floating point number
/// - `Str`: a UTF-8 string
/// - `Array`: a heterogeneous list of values
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    /// A 64-bit floating point number.
    Num(f64),
    /// A UTF-8 string.
    Str(String),
    /// A heterogeneous array of values.
    Array(Vec<Value>),
}

impl fmt::Display for Value {
    /// Formats a value for display in the REPL or script output.
    ///
    /// Numbers are displayed as-is, strings are quoted, and arrays are
    /// displayed as comma-separated lists in square brackets.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Array(a) => {
                let items: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}
