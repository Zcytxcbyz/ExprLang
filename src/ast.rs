#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Slice {
        array: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    Variable(String),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Sequence(Vec<Expr>),
    If {
        branches: Vec<(Box<Expr>, Box<Expr>)>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        var: String,
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
        body: Box<Expr>,
    },
    Break,
    Continue,
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}
