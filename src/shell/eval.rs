use std::ffi::CString;
use std::env;

use anyhow::{Result, anyhow};
use string_cache::DefaultAtom as Atom;
use super::parser::{CommandExpr, Expr, Modifier};
use crate::process::exec_external;
use crate::shell::Shell;

#[inline]
pub fn eval(exprs: Vec<Expr>, shell: &mut Shell) -> Result<()> {
    for expr in exprs {
        compute_expr(expr, shell)?;
    }
    Ok(())
}

#[inline]
fn compute_expr(ex: Expr, shell: &mut Shell) -> Result<()> {
    match ex {
        Expr::Alias(expr) => {
            shell.aliases.insert(expr.alias, expr.command);
        },
        Expr::Assign(expr) => match expr.modifier {
            Modifier::Let => {
                shell.variables.insert(expr.lhs, expr.rhs);
            },
            Modifier::Export => {
                env::set_var(expr.lhs.as_ref(), expr.rhs.as_ref());
            },
            Modifier::Alias => {
                // shell.aliases.insert(expr.lhs, expr.rhs);
            },
        },
        Expr::Command(expr) => {
            // TODO: add background commands
            let expr = unwrap_alias(&expr, shell).unwrap_or(&expr);
            match expr.command.as_ref() {
                "exit" => {
                    std::process::exit(0);
                },
                _ => {
                    // let raw_cmd = unwrap_alias(&expr.command, shell)
                    //     .unwrap_or(&expr.command);
                    let command = if expr.canonical {
                        shell.path_table.paths.get(&expr.command)
                    } else {
                        Some(&expr.command)
                    }.ok_or(anyhow!("command '{}' not found", &expr.command))?;
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
    Ok(())
}

#[inline]
fn unwrap_alias<'a>(expr: &'a CommandExpr, shell: &'a Shell) -> Option<&'a CommandExpr> {
    if let Some(cexpr) = shell.aliases.get(&expr.command) {
        // panic!("found {cexpr:#?}");
        return unwrap_alias(cexpr, shell);
    }
    Some(expr)
}
