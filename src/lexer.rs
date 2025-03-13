use crate::{errors::{self, CompilerError}, token::{self, Token}};

pub struct Lexer {
    stream: Vec<char>,
    output: Vec<Token>,
    cursor: usize,
    line: usize,
    pub errors: Vec<CompilerError>,
}

impl Lexer {
    /// Returns the current character in the stream
    fn current(&self) -> &char {
        return self.stream.get(self.cursor).unwrap_or(&'\0');
    }

    /// Returns the character that is next in the stream
    fn peek(&self) -> &char {
        return self.stream.get(self.cursor + 1).unwrap_or(&'\0');
    }

    /// Advances the lexer by 1 and returns the current character
    // fn next(&mut self) -> &char {
    //     self.cursor += 1;
    //     self.current()
    // }

    /// Takes a starting index and returns a `String` based on the stream from
    /// the starting index to the current cursor
    fn lexeme(&self, start: usize) -> String {
        if start >= self.stream.len() || self.cursor >= self.stream.len() {
            return "<EOF>".to_string();
        }
        return self.stream[start..=self.cursor].iter().collect();
    }

    fn token(&mut self, kind: token::Kind, start: usize) {
        self.output.push(Token {
            kind: kind, 
            offset: start,
            line: self.line,
            lexeme: self.lexeme(start)
        });
    }

    fn push_if_next_else(&mut self, assert_next: char, start:usize, tk_if: token::Kind, tk_else: token::Kind) {
        if self.peek() == &assert_next {
            self.cursor += 1;
            self.token(tk_if, start);
        } else {
            self.token(tk_else, start);
        }
    }
}

impl Lexer {
    pub fn new() -> Self {
        return Lexer {
            stream: vec![],
            output: vec![],
            cursor: 0usize,
            line: 1usize,
            errors: vec![],
        }
    }

    /// Clears the `stream` field of the lexer and reads in
    /// a new string as a `Vec<char>`
    pub fn load_string(&mut self, string: &String) {
        self.stream.clear();
        self.stream = string.chars().collect();
    }

    pub fn print_tokens(&self) { dbg!(&self.output); }

    pub fn scan(&mut self) {
        'scan: loop {
            let start = self.cursor;

            match self.current() {
                // Handle whitespace and newlines here
                ' ' | '\t' | '\r' => {
                    self.cursor += 1;
                    continue 'scan;
                }
                '\n' => {
                    self.token(token::Kind::Newline, start);
                    self.line += 1;
                }

                // Grouping operators
                '(' => self.token(token::Kind::LParen, start),
                ')' => self.token(token::Kind::RParen, start),
                '{' => self.token(token::Kind::LCurl, start),
                '}' => self.token(token::Kind::RCurl, start),
                '[' => self.token(token::Kind::LBrac, start),
                ']' => self.token(token::Kind::RBrac, start),

                // Arithmetic operators
                '+' => self.token(token::Kind::Plus, start),
                '-' => self.token(token::Kind::Minus, start),
                '*' => self.token(token::Kind::Star, start),
                '/' => self.token(token::Kind::Slash, start),
                '%' => self.token(token::Kind::Modulo, start),

                // '!' => self.push_if_next_else('=', start, Token::Kind::BangEqual, Token::Kind::Bang),
                // '=' => self.push_if_next_else('=', start, Token::Kind::EqualEqual, Token::Kind::Equal),

                // Literals
                'a'..='z' | 'A'..='Z' | '_' => {
                    while self.peek().is_alphanumeric() || self.peek() == &'_' {
                        self.cursor += 1;
                    }
                    let lexeme = self.lexeme(start);
                    
                    // Check if this keyword is a symbol
                    if let Some(kind) = token::Kind::get_keyword(&lexeme) {
                        // If so, push the keyword
                        self.output.push(Token {
                            kind, 
                            offset: start,
                            line: self.line,
                            lexeme,
                        });
                    } else {
                        // Otherwise, push a symbol
                        self.token(token::Kind::Symbol, start);
                    }
                }
                '0'..='9' => {
                    let mut floating_point = false;
                    while self.peek().is_ascii_digit() || self.peek() == &'_' || self.peek() == &'.' {
                        floating_point = self.peek() == &'.' && !floating_point || floating_point;
                        self.cursor += 1;
                    }

                    if floating_point {
                        self.token(token::Kind::Float, start);
                    } else {
                        self.token(token::Kind::Integer, start);
                    }
                }
            
                // EOF case
                '\0' => {
                    self.token(token::Kind::End, start);
                    break 'scan;
                }
                _ => {
                    println!("New Error");
                    println!("{} {}", self.cursor, self.current());
                    self.errors.push(CompilerError::new(errors::Kind::SyntaxError, errors::Flag::Abort, self.line, self.cursor, 1, "Illegal character"));
                }
            }

            self.cursor += 1;
        }
    }
}