use std::fmt::Display;

use crate::token;

#[derive(Debug)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub stop: usize,
    pub valid: bool,
}

impl Span {
    pub fn new(line: usize, start: usize, stop: usize) -> Self {
        Self { line, start, stop, valid: true }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AstOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Reassign,
}

impl AstOp {
    pub fn from_token(token: &token::Kind) -> Option<Self> {
        match token {
            token::Kind::Plus => Some(AstOp::Plus),
            token::Kind::Minus => Some(AstOp::Minus),
            token::Kind::Star => Some(AstOp::Multiply),
            token::Kind::Slash => Some(AstOp::Divide),
            token::Kind::Modulo => Some(AstOp::Modulus),
            token::Kind::Arrow => Some(AstOp::Reassign),
            _ => None
        }
    }

    pub fn precedence(&self) -> i8 {
        match self {
            AstOp::Reassign => -1,
            AstOp::Plus => 0,
            AstOp::Minus => 0,
            AstOp::Multiply => 1,
            AstOp::Divide => 1,
            AstOp::Modulus => 1,
        }
    }
}

impl Display for AstOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstOp::Plus => write!(f, "PLUS '+'"),
            AstOp::Minus => write!(f, "MINUS '-'"),
            AstOp::Multiply => write!(f, "MULTIPLY '*'"),
            AstOp::Divide => write!(f, "DIVIDE '/'"),
            AstOp::Modulus => write!(f, "MODULUS '%'"),
            AstOp::Reassign => write!(f, "REASSIGN '->'"),
        }
    }
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
    Grouping {
        span: Span,
        inner: Box<Expr>,
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
    },
    FunctionCall {
        span: Span,
        callee: Box<Expr>,
        arguments: Vec<Box<Expr>>,
    },
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
