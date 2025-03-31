use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::{common::{ast::{Node, NodeKind}, error::{ErrorBase, Reporter}, token::TokenKind}, Token};

use super::lexer::Lexer;

pub(crate) struct Parser<'a> {
    pub tree: Vec<Node<'a>>,
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    input: Vec<Token<'a>>,
    current: Token<'a>,
    eof: Token<'a>,
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

        let eof = input.pop().unwrap(); // this will always be EOF
        input.reverse();
        let current = input.pop().unwrap();

        return Ok(Parser {
            tree: vec![],
            reporter,
            input,
            current,
            eof,
        });
    }

    pub(crate) fn parse(&mut self) {
        while !self.input.is_empty() {
            if let Some(n) = self.parse_assignment() {
                self.tree.push(n);
            }

            _ = self.next(1);
        }
        
        if cfg!(debug_assertions) {
            println!("{:#?}", self.tree);
        }
    }

    fn peek(&self) -> &Token<'a> {
        return self.input.last().unwrap_or(&self.eof);
    }

    fn next(&mut self, n: usize) -> &Token<'a> {
        for _ in 0..n {
            self.current = self.input.pop().unwrap_or(self.eof.clone());
        }
        return &self.current;
    }
}

impl<'a> Parser<'a> {
    
    /// Matches the current token to look for some kind of nonterminal. In this case, nonterminal refers to
    /// whether or not the higher level parsers would give any more calls to precedence-aligned parsers, which `parse_literal` won't. This method is the end of the line for the precedence-aligned parsers and only makes method calls to helper functionality.
    /// 
    /// Things considered a nonterminal in this case include all literal values, including things like
    /// matrices, arrays, etc.
    fn parse_literal(&mut self) -> Option<Node<'a>> {
        let t = &self.current;
        match t.kind {
            TokenKind::LiteralString => return Some(Node::str(t)),
            TokenKind::Identifier => return Some(Node::ident(t)),
            
            TokenKind::LiteralFloat => {
                match Node::float(t) {
                    Ok(n) => return Some(n),
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t.clone() };
                        let mut r = self.reporter.borrow_mut();
                        r.error(eb, false, "could not parse float literal");
                        return None;
                    }
                }
            }
            TokenKind::LiteralInt => {
                match Node::int(t) {
                    Ok(n) => return Some(n),
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t.clone() };
                        let mut r = self.reporter.borrow_mut();
                        r.error(eb, false, "could not parse integer literal");
                        return None;
                    }
                }
            }
            _ => {
                let eb = ErrorBase::SyntaxError { token: t.clone() };
                let mut r = self.reporter.borrow_mut();
                r.error(eb, false, "expected an expression here");
                return None;
            }
        }
    }

    /// Begins by getting an lhs value with a call to parse_term, then checks if lookahead is a binary operator.
    /// If so, will consume the operator and get a value with a call to `parse_assignment`. Returns the
    /// expression as `ExprBinary`
    fn parse_factor(&mut self) -> Option<Node<'a>> {
        let mut expr = self.parse_literal()?;

        if self.peek().kind == TokenKind::Plus || self.peek().kind == TokenKind::Minus {
            self.next(1); // consume operator
            let line = self.current.line;
            let offset = self.current.offset;
            let op = self.current.kind.clone();
            self.next(1); // go next

            let rhs = self.parse_factor()?;
            let nk = NodeKind::ExprBinary { lhs: Box::new(expr), op, rhs: Box::new(rhs) };
            expr = Node::new(nk, line, offset);
        }

        return Some(expr);
    }

    /// Begins by getting a nonterminal and checks if lookahead is ARROW. If so, will consume ARROW
    /// and parse the value with a recursive call to itself. Returns the `ExprAssignment`.
    fn parse_assignment(&mut self) -> Option<Node<'a>> {
        let mut expr = self.parse_factor()?;

        // this needs some stupid garbage to avoid simultaneous mutable borrows
        // just storing everything in local variables without directly owning or referencing a token
        if self.peek().kind == TokenKind::Arrow {
            self.next(1); // consume ARROW
            let line = self.current.line;
            let offset = self.current.offset;
            let op = self.current.kind.clone();
            self.next(1); // go next

            let val = self.parse_assignment()?;
            let nk = NodeKind::ExprAssignment { id: Box::new(expr), op, val: Box::new(val) };
            expr = Node::new(nk, line, offset);
        }

        return Some(expr);
    }
}