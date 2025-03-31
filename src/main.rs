use std::{ cell::RefCell, env, fs, rc::Rc };

use common::token::Token;

mod frontend;
mod common;

fn src_by_lines(source: &String) -> Vec<String> {
    let src = source.clone();
    let lines: Vec<String> = src
        .lines()
        .map(|l| l.to_string())
        .collect();
    return lines;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &String = &args.get(1).unwrap_or_else(|| {
        eprintln!("No file path specified!");
        std::process::exit(1);
    });

    let file = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error reading file from path: {}", path);
        std::process::exit(1);
    });

    // Split the file into lines
    let lines = src_by_lines(&file);

    // Initialize the error reporter
    let reporter = Rc::new(RefCell::new(common::error::Reporter::new(&lines, path)));

    // Initialize the lexer and parser
    let mut lexer = frontend::lexer::Lexer::new(&lines, reporter.clone());
    let mut parser = frontend::parser::Parser::new(lexer, reporter.clone());

    if parser.is_err() {
        std::process::exit(0);
    }
    parser.unwrap();

    reporter.borrow_mut().print_all();
}
