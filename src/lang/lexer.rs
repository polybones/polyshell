use std::str::Chars;

use anyhow::Result;
use string_cache::DefaultAtom as Atom;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Identifier,
            value: TokenValue::None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Assignment,
    Identifier,
    Str,
    EndStatement,
    Eof,
}

#[derive(Debug)]
pub enum TokenValue {
    None,
    // Number(f64),
    String(Atom),
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            tokens.push(self.next_token()?);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.cursor = self.offset();
        let (kind, value) = self.next_kind()?;
        Ok(Token {
            kind,
            value,
        })
    }

    fn next_kind(&mut self) -> Result<(TokenKind, TokenValue)> {
        while let Some(ch) = self.chars.next() {
            match ch {
                ';' | '\n' => return Ok((TokenKind::EndStatement, TokenValue::None)),
                '=' => {
                    self.chars.next();
                    return Ok((TokenKind::Assignment, TokenValue::None));
                },
                // Str
                '"' => {
                    self.chars.next();
                    loop {
                        match self.peek() {
                            Some('\n') => {
                                panic!("lexer error: newlines are not allowed in strings");
                            },
                            Some('"') => {
                                self.chars.next();
                                break
                            },
                            None => {
                                panic!("lexer error: string bug, need to fix.")
                            },
                            _ => {
                                self.chars.next();
                            },
                        }
                    }
                    let slice = &self.source[self.cursor+1..self.offset()-1];
                    return Ok((
                        TokenKind::Str,
                        TokenValue::String(Atom::from(slice)),
                    ));
                },
                // Identifier
                ch if ch.is_ascii_alphabetic() || is_str_acceptable(ch) => {
                    while let Some(ch) = self.peek() {
                        // if ch.is_ascii_alphanumeric() || is_str_acceptable(ch) {
                        if ch != '\n' && !ch.is_whitespace() {
                            self.chars.next();
                        }
                        else {
                            break
                        }
                    }
                    let slice = &self.source[self.cursor..self.offset()];
                    return Ok((
                        TokenKind::Identifier,
                        TokenValue::String(Atom::from(slice)),
                    ));
                },
                ch if ch.is_whitespace() => {
                    if self.cursor + 1 != self.chars.as_str().len() {
                        self.cursor += 1;
                    }
                },
                _ => panic!("lexer error: unknown token \"{}\"", ch),
            }
        }
        Ok((TokenKind::Eof, TokenValue::None))
    }

    #[inline]
    fn offset(&mut self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline]
    fn is_eof(&mut self) -> bool {
        self.chars.as_str().len() == 0
    }
}

#[inline]
fn is_str_acceptable(ch: char) -> bool {
    ch == '/' || ch == ':' || ch == '-' || ch == '='
}
