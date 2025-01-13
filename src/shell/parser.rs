use anyhow::{Result, anyhow};
use string_cache::DefaultAtom as Atom;
use super::token::{Kind, Token};

#[derive(Debug)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Command(Box<CommandExpr>),
    Eof,
}

#[derive(Debug)]
pub struct AssignExpr {
    pub modifier: Modifier,
    pub lhs: Atom,
    pub rhs: Atom,
}

#[derive(Debug)]
pub enum Modifier {
    Let,
    Export,
    Alias,
}

#[derive(Debug)]
pub struct CommandExpr {
    pub command: Atom,
    pub args: Vec<Atom>,
    pub canonical: bool,
}

pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();
        while !self.tokens.is_empty() {
            exprs.push(self.next_expr()?);
        }
        Ok(exprs)
    }
    
    fn next_expr(&mut self) -> Result<Expr> {
        while let Some(tk) = self.next_token() {
            match tk.kind {
                Kind::StringLiteral => {
                    if let Some(modifier) = self.modifier(&tk) {
                        // TEMP: Will clean up this eventually
                        let lhs = self.next_token().unwrap();
                        if lhs.kind != Kind::StringLiteral {
                            return Err(anyhow!("expected 'StringLiteral' after '{:?}'", modifier));
                        }
                        if self.next_token().unwrap().kind != Kind::Eq {
                            return Err(anyhow!("expected '=' after '{}'", self.token_value(&lhs)));
                        }

                        let rhs = self.next_token().unwrap();
                        return Ok(Expr::Assign(Box::new(AssignExpr {
                            modifier,
                            lhs: Atom::from(self.token_value(&lhs)),
                            rhs: Atom::from(self.token_value(&rhs)),
                        })));
                    }
                    else {
                        let mut args: Vec<Atom> = Vec::new();
                        while self.peek().kind == Kind::StringLiteral {
                            let tk = self.next_token().unwrap();
                            args.push(Atom::from(self.token_value(&tk)));
                        
                        }
                        let tk_val = self.token_value(&tk);
                        return Ok(Expr::Command(Box::new(CommandExpr {
                            command: Atom::from(tk_val),
                            args,
                            // FIXME: Temporary implementation; will be fixed later
                            canonical: !tk_val.starts_with('/') && !tk_val.starts_with('.'),
                        })));
                    }
                },
                _ => {},
            }
        }
        Ok(Expr::Eof)
    }

    fn next_token(&mut self) -> Option<Token> {
        if !self.tokens.is_empty() {
            Some(self.tokens.remove(0))
        }
        else {
            None
        }
    }

    #[inline]
    fn peek(&self) -> Token {
        self.tokens.clone().remove(0)
    }

    #[inline]
    fn token_value(&self, token: &Token) -> &str {
        &self.source[token.start..token.end]
    }

    fn modifier(&self, token: &Token) -> Option<Modifier> {
        match self.token_value(token) {
            "let" => Some(Modifier::Let),
            "export" => Some(Modifier::Export),
            "alias" => Some(Modifier::Alias),
            _ => None,
        }
    }
}
