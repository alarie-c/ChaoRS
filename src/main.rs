use std::{fs, env};

use lexer::Lexer;

mod token;
mod lexer;
mod ast;
mod errors;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &String = {
        if args.len() >= 2 {
            &args[1]
        } else {
            eprintln!("File path was not specified...");
            std::process::exit(1);
        }
    };

    let file = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error reading file from path: {}", path);
        std::process::exit(1);
    });

    println!("File successfully fetched:");
    println!("{file}");

    // Create lexer and load the source code
    let mut lexer = Lexer::new();
    lexer.load_string(&file);
    lexer.scan();

    lexer.print_tokens();

    for error in lexer.errors {
        error.print(&file, &path);
    }
}
