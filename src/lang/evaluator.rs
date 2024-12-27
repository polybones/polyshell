use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

use bumpalo::boxed::Box;
use string_cache::DefaultAtom as Atom;

use crate::lang::parser::{Node, Program};
use crate::process;

#[derive(Debug, Default)]
pub struct Context<'ctx> {
    assignments: HashMap<Atom, Node<'ctx>>,
}

pub fn eval_program<'ctx>(program: Program<'ctx>, ctx: &mut Context<'ctx>) {
    let mut iter = program.body.into_iter();
    while let Some(node) = iter.next() {
        match node {
            Node::Assignment(assign) => {
                let inner = Box::into_inner(assign);
                match inner.lhs {
                    Node::Str(str) => {
                        ctx.assignments.insert(
                            Box::into_inner(str).atom,
                            inner.rhs,
                        );
                    },
                    _ => {},
                }
            },
            Node::Command(cmd) => {
                let inner = Box::into_inner(cmd);
                match inner.command {
                    Node::Str(str) => {
                        let c_name: &str = &str.atom;
                        let argv: Vec<CString> = inner.args.iter().map(|n| match n {
                            Node::Str(str) => {
                                let s: &str = &str.atom;
                                let cs = CString::new(s).unwrap();
                                cs
                            },
                            _ => unreachable!(),
                        }).collect();
                        process::exec_external(c_name, argv);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
