use std::iter::Peekable;
use std::vec::IntoIter;

use anyhow::{anyhow, Result};
use bumpalo::boxed::Box;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use string_cache::DefaultAtom as Atom;

use super::lexer::{Token, TokenKind, TokenValue};

#[derive(Debug)]
pub struct Program<'a> {
    pub body: BumpVec<'a, Node<'a>>,
}

#[derive(Debug)]
pub enum Node<'a> {
    Assignment(Box<'a, AssignmentNode<'a>>),
    Command(Box<'a, CommandNode<'a>>),
    Str(Box<'a, StrNode>),
    Alias(Box<'a, AssignmentNode<'a>>),
    End,
}

#[derive(Debug)]
pub struct AssignmentNode<'a> {
    pub lhs: StrNode,
    pub rhs: Node<'a>,
}

#[derive(Debug)]
pub struct CommandNode<'a> {
    pub command: StrNode,
    pub args: BumpVec<'a, Node<'a>>,
}

#[derive(Debug)]
pub struct StrNode {
    pub atom: Atom,
}

#[inline]
pub fn parse<'a>(bump: &'a Bump, tokens: Vec<Token>) -> Result<Program<'a>> {
    let mut program = Program {
        body: BumpVec::new_in(bump),
    };
    let mut iter = tokens.into_iter().peekable();
    while let Some(tk) = iter.next() {
        program.body.push(parse_node(bump, tk, &mut iter)?);
    }
    Ok(program)
}

fn parse_node<'a>(
    bump: &'a Bump,
    token: Token,
    iter: &mut Peekable<IntoIter<Token>>,
) -> Result<Node<'a>> {
    match token.kind {
        TokenKind::EndStatement => return Ok(Node::End),
        TokenKind::Str => {
            let atom = match token.value {
                TokenValue::String(str) => str,
                // SAFETY: Always has TokenValue::String
                _ => unreachable!(),
            };
            match &atom as &str {
                "alias" => {
                    if let Some(rhs) = iter.next() {
                        return Ok(Node::Alias(Box::new_in(
                            AssignmentNode {
                                lhs: StrNode { atom },
                                rhs: parse_node(bump, rhs, iter)?,
                            },
                            bump,
                        )));
                    } else {
                        return Err(anyhow!("incomplete 'alias' assignment"));
                    }
                }
                _ => {}
            };
            if let Some(tk) = iter.peek() {
                match tk.kind {
                    TokenKind::Assignment => {
                        iter.next();
                        if iter.peek().is_some() {
                            return Ok(Node::Assignment(Box::new_in(
                                AssignmentNode {
                                    lhs: StrNode { atom },
                                    // SAFETY: iter.next() is Some(...)
                                    rhs: parse_node(bump, iter.next().unwrap(), iter)?,
                                },
                                bump,
                            )));
                        }
                    }
                    // TODO: add number support for lexer, parser
                    TokenKind::Str => {
                        let mut args: BumpVec<Node<'a>> = BumpVec::new_in(bump);
                        let next_tk = iter.next().unwrap();
                        match next_tk.kind {
                            TokenKind::Str => {
                                args.push(Node::Str(Box::new_in(
                                    StrNode {
                                        atom: match next_tk.value {
                                            TokenValue::String(str) => str,
                                            _ => unreachable!(),
                                        },
                                    },
                                    bump,
                                )));
                            }
                            _ => unreachable!(),
                        }
                        loop {
                            match iter.peek() {
                                Some(ltk) => match ltk.kind {
                                    TokenKind::Str => {
                                        args.push(Node::Str(Box::new_in(
                                            StrNode {
                                                atom: match iter.next().unwrap().value {
                                                    TokenValue::String(s) => s,
                                                    _ => unreachable!(),
                                                },
                                            },
                                            bump,
                                        )));
                                    }
                                    _ => break,
                                },
                                None => break,
                            }
                        }
                        return Ok(Node::Command(Box::new_in(
                            CommandNode {
                                command: StrNode { atom },
                                args,
                            },
                            bump,
                        )));
                    }
                    _ => {}
                }
            }
            return Ok(Node::Str(Box::new_in(StrNode { atom }, bump)));
        }
        TokenKind::Eof => return Ok(Node::End),
        _ => Err(anyhow!("error processing token {:?}", token.kind)),
    }
}
