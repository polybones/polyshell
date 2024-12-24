use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Assignment,
    Eof,
}

// Adapted from https://oxc-project.github.io/javascript-parser-in-rust/docs/lexer
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tk = self.next_token();
            if tk.kind == TokenKind::Eof {
                break
            }
            tokens.push(tk);
        }
        tokens
    }

    fn next_kind(&mut self) -> TokenKind {
        while let Some(ch) = self.chars.next() {
            match ch {
                ':' if self.peek() == Some('=') => {
                    self.chars.next();
                    return TokenKind::Assignment;
                },
                '"' => {
                    loop {}
                },
                _ => {},
            }
        }
        TokenKind::Eof
    }

    fn next_token(&mut self) -> Token {
        let offset = self.offset();
        let kind = self.next_kind();
        let size = self.offset();
        Token { kind, offset, size, }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}
