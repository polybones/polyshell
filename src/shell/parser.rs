use anyhow::{Result, anyhow};
use string_cache::DefaultAtom as Atom;
use super::token::{Kind, Token};

#[derive(Debug)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Alias(Box<AliasExpr>),
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
pub struct AliasExpr {
    pub alias: Atom,
    pub command: CommandExpr,
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
                        let lhs = self.next_token();
                        if lhs.is_none() || lhs.as_ref().unwrap().kind != Kind::StringLiteral {
                            return Err(anyhow!("expected 'StringLiteral' after '{:?}'", modifier));
                        }
                        if self.next_token().is_none() {
                            return Err(anyhow!("expected '=' after '{}'", self.token_value(&lhs.unwrap())));
                        }
                        if let Some(tk) = self.next_token() {
                            if tk.kind != Kind::StringLiteral {
                                return Err(anyhow!("expected string literal"));
                            }
                            
                            let mut args = Vec::new();
                            while let Some(tk) = self.next_token() {
                                if tk.kind == Kind::EndStmt || tk.kind == Kind::Eof {
                                    break
                                }
                                else {
                                    args.push(&self.source[tk.start..tk.end]);
                                }
                            }

                            return Ok(match modifier {
                                Modifier::Alias => {
                                    Expr::Alias(Box::new(AliasExpr {
                                        alias: Atom::from(self.token_value(&lhs.unwrap())),
                                        command: CommandExpr {
                                            args: args.iter().map(|arg| Atom::from(*arg)).collect(),
                                            command: Atom::from(self.token_value(&tk)),
                                            canonical: is_canonical(self.token_value(&tk)),
                                        },
                                    }))
                                },
                                _ => Expr::Assign(Box::new(AssignExpr {
                                    modifier,
                                    lhs: Atom::from(self.token_value(&lhs.unwrap())),
                                    rhs: Atom::from(args.into_iter().collect::<String>()),
                                })),
                            });
                        }
                        return Err(anyhow!("none"));
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
                            canonical: is_canonical(tk_val),
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

#[inline]
fn is_canonical(path: &str) -> bool {
    // FIXME: Temporary implementation; will be fixed later
    !path.starts_with('/') && !path.starts_with('.')
}
