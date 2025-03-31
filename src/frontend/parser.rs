use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::{common::{ast::{Node, NodeKind}, error::Reporter, token::TokenKind}, Token};

use super::lexer::Lexer;

pub(crate) struct Parser<'a> {
    pub syntax_tree: Vec<Node<'a>>,
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    input: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(mut lexer: Lexer<'a>, reporter: Rc<RefCell<Reporter<'a>>>) -> Result<Parser<'a>, ()> {
        lexer.scan();
        let mut input = Vec::<Token>::new();
        let _ = std::mem::replace(&mut input, lexer.tokens);

        // (debug) print tokens in debug
        if cfg!(debug_assertions) {
            println!("{:#?}", input);
        }

        // check if the file is empty
        if input.is_empty() {
            return Err(());
        }

        return Ok(Parser {
            syntax_tree: vec![],
            reporter,
            input,
        });
    }
}

impl<'a> Parser<'a> {
    
}