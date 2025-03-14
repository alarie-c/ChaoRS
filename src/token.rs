#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    LParen,
    RParen,
    LCurl,
    RCurl,
    LBrac,
    RBrac,

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    Arrow,
    Comma,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,

    Integer,
    Float,
    String,
    Symbol,

    // Keywords
    Mut,
    Function,

    // Miscellaneous
    Semicolon,
    Newline,
    End,
}

impl Kind {
    pub fn get_keyword(string: &String) -> Option<Kind> {
        match string.as_str() {
            "mut" => Some(Kind::Mut),
            "function" => Some(Kind::Function),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub offset: usize,
    pub line: usize,
    pub lexeme: String,
}
