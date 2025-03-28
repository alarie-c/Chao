use super::token::{Token, TokenKind};

/// Stores the source code information for this node
/// `line`, `offset`, `len`
type NodeSpan = (usize, usize, usize);

/// Returns a NodeSpan from a token
pub(crate) fn tk_span(t: &Token) -> NodeSpan {
    return (t.line, t.offset, t.lexeme.len());
}

pub(crate) static UNARY_NONTERMINALS: [TokenKind; 1] = [
    TokenKind::Minus
];

pub(crate) static BINARY_NONTERMINALS: [TokenKind; 2] = [
    TokenKind::Plus,
    TokenKind::Minus,
];

pub(crate) static ASSIGNMENT_NONTERMINALS: [TokenKind; 1] = [
    TokenKind::Arrow,
];

#[derive(Debug)]
pub(crate) enum NodeKind {
    Ident(String),
    String(String),
    Integer(i32),
    Float(f32),
    True,
    False,
    Nil,

    Binary {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },

    Unary {
        op: TokenKind,
        operand: Box<Node>,
    },

    Assignment {
        lhs: Box<Node>,
        op: TokenKind,
        value: Box<Node>,
    },

    Invalid,
}

#[derive(Debug)]
pub(crate) struct Node {
    kind: NodeKind,
    span: NodeSpan,
}

impl Node {
    pub(crate) fn new(kind: NodeKind, span: NodeSpan) -> Node {
        return Node {
            kind,
            span,
        };
    }

    pub(crate) fn invalid(span: NodeSpan) -> Node {
        return Node {
            kind: NodeKind::Invalid,
            span,
        };
    }
}
