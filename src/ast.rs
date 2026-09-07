#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
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
        cond: Box<Expr>,
        then: Box<Expr>,
        else_branch: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
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
