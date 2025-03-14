use crate::{ ast::{ Expr, Span }, errors::{ self, CompilerError }, token::{ self, Token } };

pub struct Parser {
    stream: Vec<Token>,
    tree: Vec<Expr>,
    cursor: usize,
    pub errors: Vec<CompilerError>,
}

impl Parser {
    pub fn new(stream: Vec<Token>) -> Self {
        Self {
            stream,
            tree: vec![],
            cursor: 0usize,
            errors: vec![],
        }
    }

    fn current(&self) -> &Token {
        return self.stream.get(self.cursor).unwrap_or(self.stream.last().unwrap());
    }

    fn peek(&self) -> &Token {
        return self.stream.get(self.cursor + 1).unwrap_or(self.stream.last().unwrap());
    }

    fn span(&self) -> (usize, usize, usize) {
        let t = self.current();
        (t.line, t.offset, t.offset + t.lexeme.len() - 1)
    }
}

impl Parser {
    fn primary(&mut self) -> Expr {
        let (line, start, stop) = self.span();
        let mut span = Span::new(start, stop);

        let token = self.current();
        match token.kind {
            token::Kind::Integer => {
                let value: i64 = token.lexeme.parse().unwrap_or_else(|_| {
                    self.errors.push(
                        CompilerError::new(
                            errors::Kind::ParseError,
                            errors::Flag::Abort,
                            line,
                            start,
                            stop - start,
                            "there was a compiler error parsing this integer literal."
                        )
                    );
                    return 0i64;
                });
                return Expr::Integer { span, value };
            }
            token::Kind::Float => {
                let value: f64 = token.lexeme.parse().unwrap_or_else(|_| {
                    self.errors.push(
                        CompilerError::new(
                            errors::Kind::ParseError,
                            errors::Flag::Abort,
                            line,
                            start,
                            stop - start,
                            "there was a compiler error parsing this float literal."
                        )
                    );
                    return 0.0f64;
                });
                return Expr::Float { span, value };
            }
            _ => {
                self.errors.push(
                    CompilerError::new(
                        errors::Kind::SyntaxError,
                        errors::Flag::Abort,
                        line,
                        start,
                        stop - start,
                        "expected an expression here."
                    )
                );
                span.valid = false;
                return Expr::Empty { span };
            }
        }
    }

    pub fn parse(&mut self) {
        let e = self.primary();
        dbg!(&e);
    }
}
