use std::{ cell::RefCell, iter::Peekable, ops::Index, rc::Rc };
use crate::common::{
    ast::{ tk_span, Node, NodeKind, ASSIGNMENT_NONTERMINALS, BINARY_NONTERMINALS, UNARY_NONTERMINALS },
    error::{ ErrorBase, Reporter },
    token::{ Token, TokenKind },
};
use super::lexer::{Lexer, EOF};

pub(crate) struct Parser<'a> {
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    current: Token<'a>,
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        lexer: Peekable<Lexer<'a>>,
        reporter: Rc<RefCell<Reporter<'a>>>
    ) -> Parser<'a> {
        return Parser {
            reporter,
            current: EOF.clone(),
            lexer,
        };
    }

    pub(crate) fn parse(&mut self) -> Vec<Node> {
        let mut output: Vec<Node> = vec![];
        while let Some(t) = self.lexer.next() {
            self.current = t;
            output.push(self.parse_expr());
        }
        return output;
    }

    /// Returns `Some` peeked token if and only if that peeked `TokenKind` is contained in the array provided,
    /// will return `None` in all other cases
    fn expect<const N: usize>(&mut self, kinds: [TokenKind; N]) -> Option<TokenKind> {
        if let Some(t) = self.lexer.peek() {
            if kinds.contains(&t.kind) {
                return Some(self.lexer.next().unwrap().kind.clone());
            }
        }
        return None;
    }
}

impl<'a> Parser<'a> {
    
    /// Returns the current token as a primary expression, including
    /// `LiteralString`, `LiteralInt`, `LiteralFloat`, `Identifier`, and boolean literals.
    /// Will throw `SyntaxError` and return `Invalid` if the current token isn't a primary expression.
    fn parse_prmy(&mut self) -> Node {
        let t = &self.current;
        match t.kind {
            TokenKind::LiteralString => {
                let v = String::from(t.lexeme);
                let s = tk_span(&t);
                return Node::new(NodeKind::String(v), s);
            }
            TokenKind::LiteralInt => {
                // Remove any underbars that may be present
                let l = t.lexeme.replace("_", "");
                let s = tk_span(&t);
                let v = match l.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t.clone() };
                        self.reporter.borrow_mut().error(eb, false, "invalid integer literal");
                        return Node::new(NodeKind::Integer(0), s);
                    }
                };
                return Node::new(NodeKind::Integer(v), s);
            }
            TokenKind::LiteralFloat => {
                // Remove any underbars that may be present
                let l = t.lexeme.replace("_", "");
                let s = tk_span(&t);
                let v = match l.parse::<f32>() {
                    Ok(v) => v,
                    Err(_) => {
                        let eb = ErrorBase::ParseError { token: t.clone() };
                        self.reporter
                            .borrow_mut()
                            .error(eb, false, "invalid floating-point literal");
                        return Node::new(NodeKind::Float(0.0), s);
                    }
                };
                return Node::new(NodeKind::Float(v), s);
            }
            TokenKind::Identifier => {
                return Node::new(NodeKind::Ident(t.lexeme.to_string()), tk_span(&t));
            }
            TokenKind::True => {
                return Node::new(NodeKind::True, tk_span(&t));
            }
            TokenKind::False => {
                return Node::new(NodeKind::False, tk_span(&t));
            }
            TokenKind::Nil => {
                return Node::new(NodeKind::Nil, tk_span(&t));
            }
            _ => {
                let eb = ErrorBase::SyntaxError { token: t.clone() };
                self.reporter.borrow_mut().error(eb, false, "expected a primary expression");
                return Node::invalid(tk_span(&t));
            }
        }
    }

    // fn parse_assignment(&mut self) -> Node {
    //     let mut expr = self.parse_prmy();

    //     if let Some(op) = self.expect(ASSIGNMENT_NONTERMINALS) {

    //     }

    //     return expr;
    // }

    fn parse_expr(&mut self) -> Node {
        return self.parse_prmy();
    }

    /*
    
    
    AST_Node *Parser::factor() {
  AST_Node *expr = this->unary();

  if (this->peek_consume_if(std::vector{Token::Type::SLASH, Token::Type::STAR,
                                        Token::Type::MODULO})) {
    Token &tk = this->current();
    int line = tk.y;
    int start = tk.x;
    int stop = tk.x + tk.lexeme.length() - 1;
    AST_Op op = *(operator_from_token(tk));
    AST_Binary *n = new AST_Binary(op, line, start, stop);

    // Get the right node
    this->pos++;
    AST_Node *right = this->factor();
    n->right = right;
    n->left = expr;
    return n;
  }
  return expr;
}

AST_Node *Parser::term() {
  AST_Node *expr = this->factor();

  if (this->peek_consume_if(
          std::vector{Token::Type::PLUS, Token::Type::MINUS})) {
    Token &tk = this->current();
    int line = tk.y;
    int start = tk.x;
    int stop = tk.x + tk.lexeme.length() - 1;
    AST_Op op = *(operator_from_token(tk));
    AST_Binary *n = new AST_Binary(op, line, start, stop);

    // Get the right node
    this->pos++;
    AST_Node *right = this->factor();
    n->right = right;
    n->left = expr;
    return n;
  }
  return expr;
}

    
    
    
    
    
    
    
    
    */
}
