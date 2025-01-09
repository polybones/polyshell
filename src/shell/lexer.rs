use std::str::Chars;

use anyhow::{Result, anyhow};
use super::token::{Kind, Token};

pub struct Lexer<'a> {
    chars: Chars<'a>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while !self.eof() {
            tokens.push(self.next_tk()?);
        }
        tokens.push(Token {
            kind: Kind::Eof,
            start: self.cursor,
            end: self.cursor,
        });
        Ok(tokens)
    }

    fn next_tk(&mut self) -> Result<Token> {
        while let Some(ch) = self.advance() {
            match ch {
                ';' => {
                    let start = self.cursor;
                    self.cursor += 1;
                    return Ok(Token {
                        kind: Kind::EndStmt,
                        start,
                        end: self.cursor,
                    });
                },
                // Eq / EqCmp
                '=' => {
                    let start = self.cursor;
                    self.cursor += 1;
                    if self.peek() == Some('=') {
                        self.advance();
                        self.cursor += 1;
                        return Ok(Token {
                            kind: Kind::EqCmp,
                            start,
                            end: self.cursor,
                        });
                    }
                    else {
                        return Ok(Token {
                            kind: Kind::Eq,
                            start,
                            end: self.cursor,
                        });
                    }
                },
                '"' => {
                    self.cursor += 1;
                    debug_assert!(!self.eof());
                    let start = self.cursor;
                    while let Some(c) = self.peek() {
                        match c {
                            '"' => {
                                break
                            },
                            '\\' => {
                                self.cursor += 1;
                                if self.peek() == Some('"') {
                                    self.advance();
                                    self.cursor += 1;
                                }
                                self.advance();
                            },
                            '\n' => return Err(anyhow!("newlines are forbidden in strings")),
                            _ => {
                                self.cursor += 1;
                                self.advance();
                            },
                        }
                    }
                    self.cursor += 1;
                    return Ok(Token {
                        kind: Kind::StringLiteral,
                        start,
                        end: self.cursor,
                    });
                },
                // Handle unquoted strings
                ch if !ch.is_ascii_whitespace() => {
                    let start = self.cursor;
                    self.cursor += 1;
                    while let Some(c) = self.peek() {
                        if c == ' ' || c == '\n' || c == ';' {
                            break
                        }
                        else {
                            self.cursor += 1;
                            self.advance();
                        }
                    }
                    return Ok(Token {
                        kind: Kind::StringLiteral,
                        start,
                        end: self.cursor,
                    });
                },
                _ => {
                    self.cursor += 1;
                },
            }
        }
        panic!("not sure why this happened")
    }

    #[inline]
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    #[inline]
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline]
    fn eof(&mut self) -> bool {
        self.chars.as_str().len() == 0
    }
}
