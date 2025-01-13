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
                },
                Modifier::Export => {
                    env::set_var(expr.lhs.as_ref(), expr.rhs.as_ref());
                },
                Modifier::Alias => {
                    shell.aliases.insert(expr.lhs, expr.rhs);
                },
            },
            Expr::Command(expr) => {
                // TODO: add background commands
                match expr.command.as_ref() {
                    "exit" => {
                        std::process::exit(0);
                    },
                    _ => {
                        let raw_cmd = shell.aliases.get(&expr.command).unwrap_or(&expr.command);
                        let command = if expr.canonical {
                            shell.path_table.paths.get(raw_cmd)
                        } else {
                            Some(raw_cmd)
                        }.ok_or(anyhow!("command '{}' not found", raw_cmd))?;
                        exec_external(
                            CString::new(command.as_ref()).unwrap(),
                            expr.args
                                .iter()
                                .map(|arg| CString::new(arg.as_ref()).unwrap())
                                .collect()
                        );
                    },
                }
            },
            _ => {},
        }
    }
    Ok(())
}
