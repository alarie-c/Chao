use crate::Token;

use super::token::TokenKind;

#[derive(Debug)]
pub(crate) enum NodeKind<'a> {
    LiteralInt {
        val: i32,
    },
    LiteralFloat {
        val: f32,
    },
    LiteralStr {
        val: String,
    },
    LiteralIdent {
        id: String,
    },
    LiteralFalse,
    LiteralTrue,
    LiteralNil,

    ExprAssignment {
        id: Box<Node<'a>>,
        op: TokenKind,
        val: Box<Node<'a>>,
    },

    ExprBinary {
        lhs: Box<Node<'a>>,
        op: TokenKind,
        rhs: Box<Node<'a>>,
    },

    ExprUnary {
        op: TokenKind,
        operand: Box<Node<'a>>,
    },

    StmtVariable {
        id: String,
        val: Box<Node<'a>>,
    },

    StmtConstant {
        id: String,
        val: Box<Node<'a>>,
    },

    StmtExpression {
        expr: Box<Node<'a>>,
    },

    Invalid {
        tk: Token<'a>,
    },
}

#[derive(Debug)]
pub(crate) struct Node<'a> {
    kind: NodeKind<'a>,
    line: usize,
    offset: usize,
}

impl<'a> Node<'a> {
    pub(crate) fn new(kind: NodeKind<'a>, line: usize, offset: usize) -> Node<'a> {
        return Node {
            kind,
            line,
            offset,
        };
    }

    pub(crate) fn invalid(tk: Token<'a>) -> Node<'a> {
        let line = tk.line;
        let offset = tk.offset;
        return Node {
            kind: NodeKind::Invalid { tk },
            line,
            offset,
        };
    }

    /// Takes a token and attempts to parse it's lexeme into an `i32`.
    /// Will remove all underscores and parse, returning `Err` if anything goes wrong.
    pub(crate) fn int(token: &Token) -> Result<Node<'a>, ()> {
        let raw = token.lexeme.replace("_", "");
        match raw.parse::<i32>() {
            Ok(val) => Ok(Node::new(NodeKind::LiteralInt { val }, token.line, token.offset)),
            Err(_) => Err(()),
        }
    }

    /// Takes a token and attempts to parse it's lexeme into an `f32`.
    /// Will remove all underscores and parse, returning `Err` if anything goes wrong.
    pub(crate) fn float(token: &Token) -> Result<Node<'a>, ()> {
        let raw = token.lexeme.replace("_", "");
        match raw.parse::<f32>() {
            Ok(val) => Ok(Node::new(NodeKind::LiteralFloat { val }, token.line, token.offset)),
            Err(_) => Err(()),
        }
    }

    /// Takes a token and returns an Identifier node where `id` is the lexeme of the token.
    pub(crate) fn ident(token: &Token) -> Node<'a> {
        return Node::new(
            NodeKind::LiteralIdent { id: token.lexeme.to_string() },
            token.line,
            token.offset
        );
    }

    /// Takes a token and returns a String where `val` is a copied and dynamiclly allocated string containing
    /// the token's lexeme
    pub(crate) fn str(token: &Token) -> Node<'a> {
        return Node::new(
            NodeKind::LiteralStr { val: token.lexeme.to_string() },
            token.line,
            token.offset
        );
    }
}
