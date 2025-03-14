use std::{ env, fs, mem };

use lexer::Lexer;
use parser::Parser;

mod token;
mod lexer;
mod ast;
mod errors;
mod parser;

const FILE_PATH: &'static str = "main.chao";

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &String = {
        if args.len() >= 2 {
            &args[1]
        } else {
            // eprintln!("File path was not specified...");
            // std::process::exit(1);
            &FILE_PATH.to_string()
        }
    };

    let file = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error reading file from path: {}", path);
        std::process::exit(1);
    });

    println!("File successfully fetched:");
    println!("{file}");

    // Create lexer and load the source code
    let mut lexer = Lexer::new(&file);
    lexer.scan();

    lexer.print_tokens();

    for error in lexer.errors {
        error.print(&file, &path);
    }

    let token_stream = mem::replace(&mut lexer.output, Vec::new());
    let mut parser = Parser::new(token_stream);
    parser.parse();

    for error in parser.errors {
        error.print(&file, &path);
    }
}
