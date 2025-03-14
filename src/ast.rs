#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub stop: usize,
    pub valid: bool,
}

impl Span {
    pub fn new(start: usize, stop: usize) -> Self {
        Self { start, stop, valid: true }
    }
}

#[derive(Debug)]
pub enum AstOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Reassign,
}

#[derive(Debug)]
pub enum Expr {
    Empty {
        span: Span,
    },

    // Literals
    Float {
        span: Span,
        value: f64,
    },
    Integer {
        span: Span,
        value: i64,
    },
    String {
        span: Span,
        value: String,
    },
    Symbol {
        span: Span,
        name: String,
    },

    // Expressions
    Binary {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: AstOp,
    },
    Unary {
        span: Span,
        op: AstOp,
        operand: Box<Expr>,
    },
    Assignment {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum Stmt {
    Binding {
        span: Span,
        mutable: bool,
        name: String,
        initializer: Option<Box<Expr>>,
        annotation: Option<Box<Expr>>,
    },
}