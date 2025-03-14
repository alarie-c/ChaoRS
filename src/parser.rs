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
        (t.line, t.offset, t.offset + t.lexeme.len())
    }
}

impl Parser {
    /// Returns a vector of expression for function call arguments
    /// START = `Starting token of arg 1`
    /// END = `RParen`
    fn function_call_arguments(&mut self) -> Vec<Box<Expr>> {
        self.cursor += 1; // consume LParen
        let mut args: Vec<Box<Expr>> = vec![];

        while self.current().kind != token::Kind::RParen {
            let (line, start, stop) = self.span();
            let expression = self.function_call(); // (NOTE) change this
            // (TODO) check to make sure expression is valid here

            args.push(Box::new(expression));

            if self.peek().kind == token::Kind::Comma {
                self.cursor += 2;
                continue;
            } else if self.peek().kind == token::Kind::RParen {
                self.cursor += 1;
                continue;
            } else {
                self.errors.push(
                    CompilerError::new(
                        errors::Kind::SyntaxError,
                        errors::Flag::Abort,
                        line,
                        start,
                        stop - start,
                        "expected ',' for more arguments or ')' to close function call"
                    )
                );
                break;
            }
        }

        return args;
    }
}

impl Parser {
    fn primary(&mut self) -> Expr {
        let (line, start, stop) = self.span();
        let mut span = Span::new(start, stop);

        let token = self.current();
        match token.kind {
            token::Kind::Newline => {
                self.cursor += 1;
                return self.primary();
            }
            token::Kind::String => {
                let value: String = token.lexeme.clone();
                return Expr::String { span, value };
            }
            token::Kind::Symbol => {
                let name: String = token.lexeme.clone();
                return Expr::Symbol { span, name };
            }
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
            token::Kind::End => {
                self.errors.push(
                    CompilerError::new(
                        errors::Kind::SyntaxError,
                        errors::Flag::Abort,
                        line,
                        start - 1,
                        1,
                        "expected an expression after this but found EOF (end of file) instead."
                    )
                );
                span.valid = false;
                return Expr::Empty { span };
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

    fn function_call(&mut self) -> Expr {
        let mut expression = self.primary();

        while self.peek().kind == token::Kind::LParen {
            let (_line, start, stop) = self.span();
            let span = Span::new(start, stop);

            self.cursor += 1;
            let arguments = self.function_call_arguments();
            expression = Expr::FunctionCall { span, callee: Box::new(expression), arguments };
        }

        return expression;
    }

    fn assignment(&mut self) -> Expr {
        let expression = self.function_call();

        while self.peek().kind == token::Kind::Arrow {
            let (_line, start, stop) = self.span();
            let span = Span::new(start, stop);

            self.cursor += 2;
            let value = self.assignment();
            return Expr::Assignment { span, lhs: Box::new(expression), rhs: Box::new(value) };
        }

        return expression;
    }

    pub fn parse(&mut self) {
        let e = self.assignment();
        dbg!(&e);
    }
}
