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
                // EndStatement
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
                // Quoted strings
                '"' => {
                    self.cursor += 1;
                    let start = self.cursor;
                    loop {
                        match self.peek() {
                            Some('"') => {
                                let end = self.cursor;
                                // skip past closing quotation
                                self.cursor += 1;
                                self.advance();
                                return Ok(Token {
                                    kind: Kind::StringLiteral,
                                    start,
                                    end,
                                });
                            },
                            Some(_) => {
                                self.cursor += 1;
                                self.advance();
                            },
                            None => return Err(anyhow!(
                                "unclosed string delimeter; expected '\"'"
                            )),
                        }
                    }
                },
                // Unquoted string literals
                ch if !ch.is_ascii_whitespace() => {
                    let start = self.cursor;
                    self.cursor += 1;
                    while let Some(c) = self.peek() {
                        if c == ' ' || c == '\n' || c == ';' || c == '=' {
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
        Err(anyhow!("unknown lexer error"))
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
