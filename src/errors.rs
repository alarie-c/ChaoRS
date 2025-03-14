use std::{ fmt::Display, io::{ stdout, Write } };

const TERM_ESC: &'static str = "\x1b[";
const TERMCOL_ERROR: &'static str = "91m";
const TERMCOL_MESSAGE: &'static str = "92m";
const TERMCOL_HIGHLIGHT: &'static str = "93m";
const TERM_RESET: &'static str = "\x1b[m";

pub enum Kind {
    SyntaxError,
    UnterminatedLiteral,
    ParseError,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_name = match self {
            Kind::SyntaxError => "Syntax Error",
            Kind::UnterminatedLiteral => "Unterminated Literal",
            Kind::ParseError => "Parse Error",
        };
        write!(f, "{TERM_ESC}{TERMCOL_HIGHLIGHT}{}{TERM_RESET}", error_name)
    }
}

pub enum Flag {
    Abort = 0,
    Warning,
    Suggestion,
}

/// `offset` and `len` represents the start and length of the offending content, the reporter will automatically fetch the relevant line data when printing the error
pub struct CompilerError {
    pub kind: Kind,
    pub flag: Flag,
    line: usize,
    offset: usize,
    len: usize,
    message: String,
}

impl CompilerError {
    pub fn new(
        kind: Kind,
        flag: Flag,
        line: usize,
        offset: usize,
        len: usize,
        message: &str
    ) -> Self {
        CompilerError {
            kind,
            flag,
            line,
            offset,
            len,
            message: message.to_string(),
        }
    }

    pub fn print(&self, source: &String, path: &String) {
        // Do some bounds checking
        if self.offset >= source.len() {
            panic!("ERROR out of bounds!");
        }

        // Get the start and end of this line
        let mut ln_start = 0usize;
        let mut ln_end = source.len() - 1;

        // Decrement backwards to find the beginning of this line
        for (i, ch) in source.char_indices().rev() {
            if i > self.offset {
                continue;
            }

            if ch == '\n' {
                ln_start = i + 1;
                break;
            }
        }

        // Increment forwards to find the end of this line
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                ln_end = i - 1;
                break;
            }
        }

        if ln_start > ln_end {
            panic!("ERROR start > end!");
        }

        let line = source[ln_start..=ln_end].to_string();

        // Get the whitespace for the underline amount
        let whitespace_len = self.offset - ln_start;
        let whitespace = " ".repeat(whitespace_len);
        let underline = "^".repeat(self.len);

        // [ERROR] ../path on line 0:
        write!(
            stdout(),
            "\n{TERM_ESC}{TERMCOL_ERROR}[ERROR]{TERM_RESET} {}:{} {}:\n",
            path,
            self.line,
            self.kind
        ).unwrap();

        // ~
        // ~ line content
        // ~ ^^^^
        write!(
            stdout(),
            "~\n~ {}\n~ {}{TERM_ESC}{TERMCOL_HIGHLIGHT}{}{TERM_RESET}\n",
            line,
            whitespace,
            underline
        ).unwrap();

        // message
        write!(
            stdout(),
            "{TERM_ESC}{TERMCOL_MESSAGE}help:{TERM_RESET} {}\n",
            self.message
        ).unwrap();

        // Flush all of this to output
        stdout().flush().unwrap();
    }
}
