//! Error types and reporting utilities for ExprLang.
//!
//! This module defines structured error types with position information
//! for better error reporting.

/// Represents a position in source code.
#[derive(Debug, Clone)]
pub struct Position {
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
    /// Byte offset from the start of the input.
    pub offset: usize,
}

impl Position {
    /// Creates a new position.
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position { line, column, offset }
    }
}

/// Represents all possible errors that can occur during execution.
#[derive(Debug, Clone)]
pub enum ExprLangError {
    /// A syntax error with position and context.
    Syntax {
        message: String,
        pos: Position,
        snippet: String,
    },
    /// A type error with position.
    TypeError {
        message: String,
        pos: Position,
    },
    /// A runtime error with optional position.
    RuntimeError {
        message: String,
        pos: Option<Position>,
    },
    /// Internal control flow signal for break/continue.
    ControlFlow {
        kind: ControlFlowKind,
    },
}

/// Control flow signals for loop management.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlowKind {
    /// Exit the innermost loop.
    Break,
    /// Skip to the next iteration.
    Continue,
}

impl std::fmt::Display for ExprLangError {
    /// Formats the error for display.
    ///
    /// Syntax errors include a code snippet with a caret pointing to the error.
    /// Type and runtime errors include position information when available.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprLangError::Syntax { message, pos, snippet } => {
                writeln!(f, "SyntaxError at line {}, column {}:", pos.line, pos.column)?;
                writeln!(f, "  {}", snippet)?;
                write!(f, "  {}^ {}", " ".repeat(pos.column - 1), message)
            }
            ExprLangError::TypeError { message, pos } => {
                write!(f, "TypeError at line {}, column {}: {}", pos.line, pos.column, message)
            }
            ExprLangError::RuntimeError { message, pos } => {
                if let Some(pos) = pos {
                    write!(f, "RuntimeError at line {}, column {}: {}", pos.line, pos.column, message)
                } else {
                    write!(f, "RuntimeError: {}", message)
                }
            }
            ExprLangError::ControlFlow { kind } => {
                write!(f, "ControlFlow: {:?}", kind)
            }
        }
    }
}

impl From<ExprLangError> for String {
    fn from(err: ExprLangError) -> String {
        err.to_string()
    }
}
