// ========== 错误类型定义 ==========

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position { line, column, offset }
    }
}

#[derive(Debug, Clone)]
pub enum ExprLangError {
    Syntax {
        message: String,
        pos: Position,
        snippet: String,
    },
    TypeError {
        message: String,
        pos: Position,
    },
    RuntimeError {
        message: String,
        pos: Option<Position>,
    },
    ControlFlow {
        kind: ControlFlowKind,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlowKind {
    Break,
    Continue,
}

impl std::fmt::Display for ExprLangError {
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
