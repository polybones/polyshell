pub mod eval;
pub mod lexer;
pub mod parser;
pub mod path_table;
pub mod token;

use std::collections::HashMap;

use anyhow::Result;
use path_table::PathTable;
use string_cache::DefaultAtom as Atom;

#[derive(Default)]
pub struct Shell {
    pub aliases: HashMap<Atom, Atom>,
    pub variables: HashMap<Atom, Atom>,
    pub path_table: PathTable,
}

#[inline]
pub fn run(source: &str, shell: &mut Shell) -> Result<()> {
    let mut lx = lexer::Lexer::new(source);
    let tks = lx.tokenize()?;
    let mut parser = parser::Parser::new(source, tks);
    let exprs = parser.parse()?;
    eval::eval(exprs, shell)?;
    Ok(())
}
