use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::iter::Peekable;

use bumpalo::boxed::Box;
use bumpalo::collections::vec::IntoIter;
use string_cache::DefaultAtom as Atom;

use crate::lang::parser::{Node, Program};
use crate::process;

#[derive(Debug)]
pub struct Context<'ctx> {
    aliases: HashMap<Atom, Node<'ctx>>,
}

impl<'ctx> Default for Context<'ctx> {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }
}

pub fn eval_program<'ctx>(program: Program<'ctx>, ctx: &mut Context<'ctx>) {
    let mut iter = program.body.into_iter().peekable();
    while let Some(node) = iter.next() {
        compute(&mut iter, node, ctx);
    }
}

#[inline]
fn compute<'ctx>(iter: &mut Peekable<IntoIter<'ctx, Node<'ctx>>>, node: Node<'ctx>, ctx: &mut Context<'ctx>) {
    match node {
        Node::Alias(alias) => {
            let inner = Box::into_inner(alias);
            match inner.rhs {
                Node::Assignment(assign) => {
                    let inner2 = Box::into_inner(assign);
                    match inner2.lhs {
                        Node::Str(str) => {
                            ctx.aliases.insert(
                                Box::into_inner(str).atom,
                                inner2.rhs,
                            );
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        },
        Node::Assignment(assign) => {
            let inner = Box::into_inner(assign);
            match inner.lhs {
                Node::Str(left_str) => {
                    match inner.rhs {
                        Node::Str(right_str) => {
                            env::set_var(
                                &left_str.atom as &str,
                                &right_str.atom as &str,
                            );
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        },
        Node::Command(_) => compute_command(node, ctx),
        Node::Str(_) => {
            match iter.peek() {
                Some(Node::End) | None => {
                    compute_command(node, ctx);
                },
                _ => {},
            }
        },
        _ => {},
    }
}

#[inline]
fn compute_command<'ctx>(node: Node<'ctx>, ctx: &mut Context<'ctx>) {
    let (command, argv): (&str, Vec<CString>) = match node {
        Node::Str(ref str) => (&str.atom as &str, Vec::with_capacity(0)),
        Node::Command(ref cmd) => {
            let args = cmd.args.iter().filter_map(|arg| match arg {
                Node::Str(str) => Some(CString::new(&str.atom as &str).unwrap()),
                _ => None,
            }).collect();
            (&cmd.command.atom as &str, args)
        },
        _ => panic!(),
    };
    if let Ok(path_var) = env::var("PATH") {
        let executable = path_var
            .split(":")
            .find_map(|path| {
                fs::read_dir(path).ok()?.find_map(|entry| {
                    let file = entry.ok()?;
                    if file.file_name().to_string_lossy() == command {
                        Some(file.path())
                    } else {
                        None
                    }
                })
            });
        if let Some(ex) = executable {
            process::exec_external(ex.to_str().unwrap(), argv);
        }
        else {
            // TODO: add better system for builtin commands/args maybe
            match command {
                "exit" => std::process::exit(0),
                _ => println!("polyshell: command \"{command}\" not found"),
            }
        }
    }
}
