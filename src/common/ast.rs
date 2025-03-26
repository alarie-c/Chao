use super::token::Token;

/// Stores the source code information for this node
/// `line`, `offset`, `len`
type NodeSpan = (usize, usize, usize);

/// Returns a NodeSpan from a token
pub(crate) fn tk_span(t: &Token) -> NodeSpan {
    return (t.line, t.offset, t.lexeme.len());
}

pub(crate) enum NodeKind {
    Ident(String),
    String(String),
    Integer(i32),
    Float(f32),
    True,
    False,
    Nil,

    Invalid,
}

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
