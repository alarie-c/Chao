use std::{cell::RefCell, rc::Rc};

use crate::common::{error::{ErrorBase, Reporter}, token::{Token, TokenKind}};

pub(crate) struct Lexer<'a> {
    pub reporter: Rc<RefCell<Reporter<'a>>>,
    pub tokens: Vec<Token<'a>>,
    input: &'a Vec<String>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a Vec<String>, reporter: Rc<RefCell<Reporter<'a>>>) -> Lexer<'a> {
        return Lexer {
            reporter,
            tokens: vec![],
            input,
        }
    }

    pub(crate) fn scan(&mut self) {
        let mut lines = self.input.iter().enumerate();

        // helper crap
        let mut last_ii = 0;
        let mut last_i = 0;

        while let Some((i, ln)) = lines.next() {
            last_i = i + 1;
            
            let i = i + 1; // shadow i because lines indicies are n - 1
            let mut chars = ln.char_indices().peekable();

            while let Some((ii, ch)) = chars.next() {
                last_ii = ii;
                
                match ch {
                    ' ' | '\t' | '\r' => {},
                    '(' => self.tokens.push(Token::new(TokenKind::LParen, ii, i, &ln[ii..ii + '('.len_utf8()])),
                    ')' => self.tokens.push(Token::new(TokenKind::LParen, ii, i, &ln[ii..ii + ')'.len_utf8()])),

                    '+' => {
                        let mut token: Option<Token> = None;
                        if let Some((_, peeked)) = chars.peek() {
                            match peeked {
                                '+' => token = Some(Token::new(TokenKind::PlusPlus, ii, i, &ln[ii..ii + "++".len()])),
                                '=' => token = Some(Token::new(TokenKind::PlusEqual, ii, i, &ln[ii..ii + "+=".len()])),
                                _ => {}
                            }
                        }
                        match token {
                            Some(_) => _ = chars.next(),
                            None => token = Some(Token::new(TokenKind::Plus, ii, i, &ln[ii..ii + '+'.len_utf8()])),
                        }
                        self.tokens.push(token.unwrap());
                    }

                    '-' => {
                        let mut token: Option<Token> = None;
                        if let Some((_, peeked)) = chars.peek() {
                            match peeked {
                                '-' => token = Some(Token::new(TokenKind::MinusMinus, ii, i, &ln[ii..ii + "--".len()])),
                                '=' => token = Some(Token::new(TokenKind::MinusEqual, ii, i, &ln[ii..ii + "-=".len()])),
                                '>' => token = Some(Token::new(TokenKind::Arrow, ii, i, &ln[ii..ii + "->".len()])),
                                _ => {}
                            }
                        }
                        match token {
                            Some(_) => _ = chars.next(),
                            None => token = Some(Token::new(TokenKind::Minus, ii, i, &ln[ii..ii + '-'.len_utf8()])),
                        }
                        self.tokens.push(token.unwrap());
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
                                        line: i,
                                        offset: ii,
                                    };
                                    let mut r = self.reporter.borrow_mut();
                                    r.error(eb, false, "Unterminated character literal");
                                    break 'char_literal;
                                }
                            }
                        }
    
                        // push the token
                        let lexeme = &ln[ii + 1..ii + len];
                        self.tokens.push(Token::new(TokenKind::LiteralChar, ii, i, lexeme));
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
                                        line: i,
                                        offset: ii,
                                    };
                                    let mut r = self.reporter.borrow_mut();
                                    r.error(eb, false, "Unterminated string literal");
                                    break 'str_literal;
                                }
                            }
                        }
    
                        // Push the token
                        let lexeme = &ln[ii + 1..ii + len];
                        self.tokens.push(Token::new(TokenKind::LiteralString, ii, i, lexeme));
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut len = ch.len_utf8();
    
                        // consume characters
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
    
                        // push the token
                        let lexeme = &ln[ii..ii + len];
                        self.tokens.push(Token::new(TokenKind::as_keyword(lexeme), ii, i, lexeme));
                    }
                    '0'..='9' => {
                        let mut len = ch.len_utf8();
    
                        // consume characters
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
    
                        // push the token as either float or integer
                        // based on prescence of decimal point
                        let lexeme = &ln[ii..ii + len];
                        if lexeme.contains(".") {
                            self.tokens.push(Token::new(TokenKind::LiteralFloat, ii, i, lexeme));
                        } else {
                            self.tokens.push(Token::new(TokenKind::LiteralInt, ii, i, lexeme));
                        }
                    }

                    _ => {
                        // (error) throw illegal character error
                        let eb = ErrorBase::IllegalCharacter { line: i, offset: ii };
                        let mut r = self.reporter.borrow_mut();
                        r.error(eb, false, "illegal character found");
                    }
                }
            }
        }
        
        // push eof
        self.tokens.push(Token::eof(last_ii, last_i));
    }
}