pub struct Span {
    pub start: usize,
    pub stop: usize,
}

pub enum AstOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Reassign,
}

pub enum Expr {
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

pub enum Stmt {
    Binding {
        span: Span,
        mutable: bool,
        name: String,
        initializer: Option<Box<Expr>>,
        annotation: Option<Box<Expr>>,
    },
}