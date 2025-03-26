use std::{ cell::RefCell, iter::Peekable, rc::Rc };
use crate::common::{
    ast::{ tk_span, Node, NodeKind },
    error::{ ErrorBase, Reporter },
    token::{ Token, TokenKind },
};
use super::lexer::Lexer;

pub(crate) struct Parser<'a> {
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    lexer: Peekable<Lexer<'a>>,
    output: Vec<Node>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        lexer: Peekable<Lexer<'a>>,
        reporter: Rc<RefCell<Reporter<'a>>>
    ) -> Parser<'a> {
        return Parser {
            reporter,
            lexer,
            output: vec![],
        };
    }
}

impl<'a> Parser<'a> {
    fn parse_prmy(&mut self, t: &'a Token) -> Node {
        match t.kind {
            TokenKind::LiteralString => {
                let v = String::from(t.lexeme);
                let s = tk_span(t);
                return Node::new(NodeKind::String(v), s);
            }
            TokenKind::LiteralInt => {
                // Remove any underbars that may be present
                let l = t.lexeme.replace("_", "");
                let s = tk_span(t);
                let v = match l.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t };
                        self.reporter.borrow_mut().error(eb, false, "invalid integer literal");
                        return Node::new(NodeKind::Integer(0), s);
                    }
                };
                return Node::new(NodeKind::Integer(v), s);
            }
            TokenKind::LiteralFloat => {
                // Remove any underbars that may be present
                let l = t.lexeme.replace("_", "");
                let s = tk_span(t);
                let v = match l.parse::<f32>() {
                    Ok(v) => v,
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t };
                        self.reporter
                            .borrow_mut()
                            .error(eb, false, "invalid floating-point literal");
                        return Node::new(NodeKind::Float(0.0), s);
                    }
                };
                return Node::new(NodeKind::Float(v), s);
            }
            TokenKind::Identifier => {
                return Node::new(NodeKind::Ident(t.lexeme.to_string()), tk_span(t));
            }
            TokenKind::True => {
                return Node::new(NodeKind::True, tk_span(t));
            }
            TokenKind::False => {
                return Node::new(NodeKind::False, tk_span(t));
            }
            TokenKind::Nil => {
                return Node::new(NodeKind::Nil, tk_span(t));
            }
            _ => {
                let eb = ErrorBase::SyntaxError { token: t };
                self.reporter.borrow_mut().error(eb, false, "expected a primary expression");
                return Node::invalid(tk_span(t));
            }
        }
    }
}
