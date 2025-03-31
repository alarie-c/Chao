use std::{ cell::RefCell, collections::VecDeque, rc::Rc };
use crate::common::{ error::{ ErrorBase, Reporter }, token::{ Token, TokenKind } };

pub(crate) static EOF: &'static Token = &Token { kind: TokenKind::Eof, offset: 0, line: 0, lexeme: "<EOF>" };

pub(crate) struct Lexer<'a> {
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    stream: &'a Vec<String>,
    buffer: VecDeque<Token<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(stream: &'a Vec<String>, reporter: Rc<RefCell<Reporter<'a>>>) -> Lexer<'a> {
        return Lexer {
            stream,
            line: 0usize,
            buffer: VecDeque::new(),
            reporter,
        };
    }

    fn token(&mut self, kind: TokenKind, offset: usize, lexeme: &'a str) {
        self.buffer.push_front(Token { kind, offset, line: self.line, lexeme });
    }

    fn scanline(&mut self) {
        let src = &self.stream[self.line - 1];
        let mut chars = src.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            match ch {
                /* Skip ignored whitepsace */
                ' ' | '\t' | '\r' => {
                    continue;
                }
                '\n' => {
                    break;
                }

                /* Real tokens here  */
                '(' => self.token(TokenKind::LParen, i, &src[i..i + '('.len_utf8()]),
                ')' => self.token(TokenKind::RParen, i, &src[i..i + ')'.len_utf8()]),
                '+' => {
                    match chars.peek() {
                        Some((_, c)) =>
                            match *c {
                                '=' => {
                                    chars.next();
                                    self.token(TokenKind::PlusEqual, i, &src[i..i + "+=".len()]);
                                    continue;
                                }
                                _ => {}
                            }
                        _ => {}
                    }
                    self.token(TokenKind::Plus, i, &src[i..i + '+'.len_utf8()]);
                }
                '\'' => {
                    let mut len = ch.len_utf8();

                    'char_literal: loop {
                        match chars.peek() {
                            Some((_, c)) => {
                                if *c == '\'' {
                                    chars.next();
                                    break 'char_literal;
                                }
                                len += chars.next().unwrap().1.len_utf8();
                            }
                            None => {
                                let eb = ErrorBase::UnterminatedLiteral {
                                    line: self.line,
                                    offset: i,
                                };
                                self.reporter
                                    .borrow_mut()
                                    .error(eb, false, "Unterminated character literal");
                                break 'char_literal;
                            }
                        }
                    }

                    // Push the token
                    let lexeme = &src[i + 1..i + len];
                    self.token(TokenKind::LiteralChar, i, lexeme);
                }
                '"' => {
                    let mut len = ch.len_utf8();

                    'str_literal: loop {
                        match chars.peek() {
                            Some((_, c)) => {
                                if *c == '"' {
                                    chars.next();
                                    break 'str_literal;
                                }
                                len += chars.next().unwrap().1.len_utf8();
                            }
                            None => {
                                let eb = ErrorBase::UnterminatedLiteral {
                                    line: self.line,
                                    offset: i,
                                };
                                self.reporter
                                    .borrow_mut()
                                    .error(eb, false, "Unterminated string literal");
                                break 'str_literal;
                            }
                        }
                    }

                    // Push the token
                    let lexeme = &src[i + 1..i + len];
                    self.token(TokenKind::LiteralString, i, lexeme);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut len = ch.len_utf8();

                    // Consume characters
                    while let Some((_, c)) = chars.peek() {
                        match *c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                len += chars.next().unwrap().1.len_utf8();
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    // Push the token
                    let lexeme = &src[i..i + len];
                    self.token(TokenKind::as_keyword(lexeme), i, lexeme);
                }
                '0'..='9' => {
                    let mut len = ch.len_utf8();

                    // Consume characters
                    while let Some((_, c)) = chars.peek() {
                        match *c {
                            '0'..='9' | '_' | '.' => {
                                len += chars.next().unwrap().1.len_utf8();
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    // Push the token as either float or integer
                    // based on prescence of decimal point
                    let lexeme = &src[i..i + len];
                    if lexeme.contains(".") {
                        self.token(TokenKind::LiteralFloat, i, lexeme);
                    } else {
                        self.token(TokenKind::LiteralInt, i, lexeme);
                    }
                }
                _ => {}
            }
        }

        // Check if the line is empty and if so, scan the next one
        if self.buffer.is_empty() && self.line < self.stream.len() {
            self.line += 1;
            self.scanline();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() && self.line < self.stream.len() {
            self.line += 1;
            self.scanline();
        }

        let t = self.buffer.pop_back();
        return t;
    }
}
