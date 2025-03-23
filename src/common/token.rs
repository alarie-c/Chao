use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub kind: TokenKind,
    pub offset: usize,
    pub line: usize,
    pub lexeme: &'a str,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} {} '{}'", self.line, self.offset, self.kind, self.lexeme)
    }
}

#[derive(Debug)]
pub(crate) enum TokenKind {
    Eof = 0,

    LParen,
    RParen,

    Plus,
    Minus,
    PlusEqual,
    MinusEqual,

    LiteralString,
    LiteralInt,
    LiteralFloat,
    LiteralChar,
    Identifier,
    True,
    False,
    Nil,
}

impl TokenKind {
    pub(crate) fn as_keyword(lexeme: &str) -> TokenKind {
        match lexeme {
            "true" => Self::True,
            "false" => Self::False,
            "nil" => Self::Nil,
            _ => Self::Identifier,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::LParen => "LParen",
            Self::RParen => "RParen",
            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::PlusEqual => "PlusEqual",
            Self::MinusEqual => "MinusEqual",
            Self::LiteralString => "String",
            Self::LiteralInt => "Integer",
            Self::LiteralFloat => "Float",
            Self::LiteralChar => "Char",
            Self::Identifier => "Ident",
            Self::True => "True",
            Self::False => "False",
            Self::Nil => "Nil",
            Self::Eof => "EOF",
        })
    }
}
