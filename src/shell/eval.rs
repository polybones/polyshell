use std::ffi::CString;
use std::env;

use anyhow::{Result, anyhow};
use super::parser::{Expr, Modifier};
use crate::process::exec_external;
use crate::shell::Shell;

pub fn eval(exprs: Vec<Expr>, shell: &mut Shell) -> Result<()> {
    for ex in exprs {
        match ex {
            Expr::Assign(expr) => match expr.modifier {
                Modifier::Let => {
                    shell.variables.insert(expr.lhs, expr.rhs);
                }
                Modifier::Export => {
                    env::set_var(expr.lhs.as_ref(), expr.rhs.as_ref());
                }
                Modifier::Alias => {
                    shell.aliases.insert(expr.lhs, expr.rhs);
                }
            },
            Expr::Command(cmd) => {
                // TODO: Handle internal commands, background commands
                let command = if cmd.canonical {
                    shell.path_table.paths.get(&cmd.command)
                        .ok_or(anyhow!("polyshell: command '{}' not found", cmd.command))
                }
                else {
                    Ok(&cmd.command)
                }?;
                exec_external(
                    CString::new(command.as_ref()).unwrap(),
                    cmd.args
                        .iter()
                        .map(|arg| CString::new(arg.as_ref()).unwrap())
                        .collect()
                );
            },
            _ => {},
        }
    }
    Ok(())
}
