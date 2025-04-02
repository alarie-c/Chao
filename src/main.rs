use std::{ cell::RefCell, env, fs, rc::Rc };
use common::{ ast::Node, token::Token };

mod frontend;
mod common;
mod analysis;

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

    let arg2: String = args.get(2).unwrap_or(&String::new()).to_owned();

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
    let lex = frontend::lexer::Lexer::new(&lines, reporter.clone());
    let par = frontend::parser::Parser::new(lex, reporter.clone());

    if par.is_err() {
        std::process::exit(0);
    }
    let mut parser = par.unwrap();
    parser.parse();

    reporter.borrow_mut().print_all();


    if arg2.is_empty() {
        std::process::exit(0);
    }

    // name and type resolution
    let mut ast = Vec::<Node>::new();
    _ = std::mem::replace(&mut ast, parser.tree);

    let mut resolver = analysis::resolver::Resolver::new();
    let resolver_errors = resolver.resolve(ast);
    if resolver_errors.is_err() {
        reporter.borrow_mut().dump(resolver_errors.unwrap_err());
        reporter.borrow_mut().print_all();
        std::process::exit(1);
    }
    
    if arg2.as_str() == "--ir" {
        //let mut ir_compiler = analysis::irgen::IrCompiler::new();
        //let ir = ir_compiler.compile(ast);
        //println!("{:#?}", ir);
    }

    reporter.borrow_mut().print_all();
}
