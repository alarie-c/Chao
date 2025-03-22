use std::collections::VecDeque;
use crate::common::token::{ Token, TokenKind };

pub(crate) struct Lexer<'a> {
    stream: &'a Vec<String>,
    line: usize,

    /// Stores the current line's tokens
    buffer: VecDeque<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(stream: &'a Vec<String>) -> Lexer<'a> {
        return Lexer {
            stream,
            line: 0usize,
            buffer: VecDeque::new(),
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
                    chars.next();
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
                        self.token(TokenKind::Float, i, lexeme);
                    } else {
                        self.token(TokenKind::Integer, i, lexeme);
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
        return self.buffer.pop_back();
    }
}
