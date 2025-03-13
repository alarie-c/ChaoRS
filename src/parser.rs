use crate::{ast::Expr, token::Token};

pub struct Parser {
    stream: Vec<Token>,
    tree: Vec<Expr>,
    cursor: usize,
}

impl Parser {
    fn current(&self) -> &Token {
        return self.stream.get(self.cursor).unwrap_or(self.stream.last().unwrap());
    }

    fn peek(&self) -> &Token {
        return self.stream.get(self.cursor + 1).unwrap_or(self.stream.last().unwrap());
    }
}