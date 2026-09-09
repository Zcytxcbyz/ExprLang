//! Defines the Abstract Syntax Tree (AST) for ExprLang expressions.
//!
//! The AST represents parsed expressions as a tree of nodes. Each node
//! corresponds to a syntactic construct in the language.

/// Represents all possible expression nodes in the AST.
///
/// Each variant corresponds to a syntactic construct in ExprLang,
/// from literals and variables to control flow and function definitions.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A floating-point literal.
    Number(f64),
    /// A string literal.
    String(String),
    /// An array literal.
    Array(Vec<Expr>),
    /// Array indexing: `array[index]`
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    /// Array slicing: `array[start:end]`
    Slice {
        array: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    /// Variable reference.
    Variable(String),
    /// Binary operation (e.g., `+`, `-`, `*`, `/`, comparisons, logic).
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation (e.g., negation `-`, logical not `!`).
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    /// Function call: `name(args...)`
    Call {
        name: String,
        args: Vec<Expr>,
    },
    /// Variable assignment: `name = expr`
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    /// A sequence of expressions separated by semicolons.
    Sequence(Vec<Expr>),
    /// Conditional expression: `if cond then expr [elif cond then expr]* [else expr]`
    If {
        branches: Vec<(Box<Expr>, Box<Expr>)>,
        else_branch: Option<Box<Expr>>,
    },
    /// While loop: `while cond do body`
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    /// For loop: `for var in start..end [step step] do body`
    For {
        var: String,
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
        body: Box<Expr>,
    },
    /// Break statement: exits the innermost loop.
    Break,
    /// Continue statement: skips to the next iteration.
    Continue,
    /// Function definition: `fn name(params) = body`
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
}

/// Binary operators supported by ExprLang.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    // Logical
    And,
    Or,
}

/// Unary operators supported by ExprLang.
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    /// Numeric negation: `-expr`
    Neg,
    /// Logical NOT: `!expr`
    Not,
}
