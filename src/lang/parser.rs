use std::iter::Peekable;
use std::vec::IntoIter;

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
    Null,
}

#[derive(Debug)]
pub struct AssignmentNode<'a> {
    pub lhs: Node<'a>,
    pub rhs: Node<'a>,
}

#[derive(Debug)]
pub struct CommandNode<'a> {
    pub command: Node<'a>,
    pub args: BumpVec<'a, Node<'a>>,
}

#[derive(Debug)]
pub struct StrNode {
    pub atom: Atom,
}

#[inline]
pub fn parse<'a>(bump: &'a Bump, tokens: Vec<Token>) -> Program<'a> {
    let mut program = Program {
        body: BumpVec::new_in(bump),
    };
    let mut iter = tokens.into_iter().peekable();
    while let Some(tk) = iter.next() {
        match parse_node(bump, tk, &mut iter) {
            Node::Null => {},
            node => program.body.push(node),
        }
    }
    program
}

fn parse_node<'a>(bump: &'a Bump, token: Token, iter: &mut Peekable<IntoIter<Token>>) -> Node<'a> {
    match token.kind {
        TokenKind::Identifier | TokenKind::Str => {
            let atom = match token.value {
                TokenValue::String(str) => str,
                _ => {
                    // SAFETY: Always has TokenValue::String
                    unreachable!()
                }
            };
            let node = Node::Str(Box::new_in(StrNode { atom }, bump));
            if let Some(tk) = iter.peek() {
                match tk.kind {
                    TokenKind::Assignment => {
                        iter.next();
                        if iter.peek().is_some() {
                            return Node::Assignment(Box::new_in(
                                AssignmentNode {
                                    lhs: node,
                                    // SAFETY: iter.next() is Some(...)
                                    rhs: parse_node(bump, iter.next().unwrap(), iter),
                                },
                                bump,
                            ));
                        }
                    },
                    // TODO: add number support for lexer, parser
                    TokenKind::Identifier | TokenKind::Str => {
                        let mut args: BumpVec<Node<'a>> = BumpVec::new_in(bump);
                        let next_tk = iter.next().unwrap();
                        match next_tk.kind {
                            TokenKind::Identifier | TokenKind::Str => {
                                args.push(Node::Str(Box::new_in(StrNode { atom:
                                    match next_tk.value {
                                        TokenValue::String(str) => str,
                                        _ => unreachable!(),
                                    }}, bump)));
                            },
                            _ => unreachable!(),
                        }
                        loop {
                            match iter.peek() {
                                Some(ltk) => match ltk.kind {
                                    TokenKind::Identifier | TokenKind::Str => {
                                        args.push(Node::Str(Box::new_in(StrNode { atom:
                                            match iter.next().unwrap().value {
                                                TokenValue::String(s) => s,
                                                _ => unreachable!(),
                                            }}, bump)));
                                    }
                                    _ => break,
                                },
                                None => break,
                            }
                        }
                        // println!("ARRRRRGHHH {:#?}", args);
                        return Node::Command(Box::new_in(
                            CommandNode {
                                command: node,
                                args,
                            }, bump));
                    },
                    _ => {},
                }
            }
            node
        }
        _ => Node::Null,
    }
}
