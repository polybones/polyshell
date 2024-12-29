use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::iter::Peekable;

use anyhow::{anyhow, Result};
use bumpalo::boxed::Box;
use bumpalo::collections::vec::IntoIter;
use string_cache::DefaultAtom as Atom;

use crate::lang::parser::{Node, Program};
use crate::process;

#[derive(Debug, Default)]
pub struct Context {
    aliases: HashMap<Atom, Vec<Atom>>,
}

pub fn eval_program<'ctx>(program: Program<'ctx>, ctx: &mut Context) -> Result<()> {
    let mut iter = program.body.into_iter().peekable();
    while let Some(node) = iter.next() {
        compute(&mut iter, node, ctx)?;
    }
    Ok(())
}

#[inline]
fn compute<'ctx>(
    iter: &mut Peekable<IntoIter<'ctx, Node<'ctx>>>,
    node: Node<'ctx>,
    context: &mut Context,
) -> Result<()> {
    match node {
        Node::Alias(alias) => {
            let inner = Box::into_inner(alias);
            match inner.rhs {
                Node::Assignment(assign) => {
                    let inner_assign = Box::into_inner(assign);
                    match inner_assign.rhs {
                        Node::Command(cmd) => {
                            let inner_cmd = Box::into_inner(cmd);
                            context.aliases.insert(
                                inner_assign.lhs.atom,
                                std::iter::once(inner_cmd.command.atom)
                                    .chain(inner_cmd.args.into_iter().filter_map(|arg| match arg {
                                        Node::Str(s_arg) => Some(Box::into_inner(s_arg).atom),
                                        _ => None,
                                    }))
                                    .collect(),
                            );
                        }
                        Node::Str(str) => {
                            let inner_str = Box::into_inner(str);
                            context
                                .aliases
                                .insert(inner_assign.lhs.atom, vec![inner_str.atom]);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Node::Assignment(assign) => {
            let inner = Box::into_inner(assign);
            match inner.rhs {
                Node::Str(right_str) => {
                    env::set_var(&inner.lhs.atom as &str, &right_str.atom as &str);
                }
                _ => {}
            }
        }
        Node::Command(cmd) => {
            compute_command(
                iter,
                std::iter::once(&cmd.command.atom as &str)
                    .chain(cmd.args.iter().filter_map(|x| match x {
                        Node::Str(str) => Some(&str.atom as &str),
                        _ => None,
                    }))
                    .collect(),
                context,
            )?;
        }
        Node::Str(str) => match iter.peek() {
            Some(Node::End) | None => {
                compute_command(iter, vec![&str.atom], context)?;
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

#[inline]
fn compute_command<'ctx>(
    iter: &mut Peekable<IntoIter<'ctx, Node<'ctx>>>,
    argv: Vec<&str>,
    context: &Context,
) -> Result<()> {
    let cmd_str: &str = argv.get(0).unwrap();
    match cmd_str {
        "exit" => std::process::exit(0),
        _ => {
            if let Some(alias_args) = context.aliases.get(&Atom::from(cmd_str)) {
                compute_command(
                    iter,
                    alias_args.iter().map(|x| x as &str).collect(),
                    context,
                )?;
                return Ok(())
            } else if let Ok(path_var) = env::var("PATH") {
                let executable = path_var.split(":").find_map(|path| {
                    fs::read_dir(path).ok()?.find_map(|entry| {
                        let file = entry.ok()?;
                        if file.file_name().to_string_lossy() == cmd_str {
                            Some(file.path())
                        } else {
                            None
                        }
                    })
                });
                if let Some(ex) = executable {
                    process::exec_external(
                        ex.to_str().unwrap(),
                        argv.iter()
                            .skip(1)
                            .map(|arg| CString::new(*arg).unwrap())
                            .collect(),
                    )?;
                    return Ok(())
                }
            }
        }
    }
    // did not find internal / external command
    Err(anyhow!("command \"{cmd_str}\" not found"))
}
