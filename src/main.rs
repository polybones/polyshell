mod lang;
mod process;
mod shell;

use anyhow::Result;
use bumpalo::Bump;

fn main() -> Result<()> {
    let bump: Bump = Bump::new();
    let tks = lang::lexer::Lexer::new("echo hi").tokenize();
    let ast = lang::parser::parse(&bump, tks);
    // println!("ast: {ast:#?}");
    let mut ctx = lang::evaluator::Context::default();
    lang::evaluator::eval_program(ast, &mut ctx);
    Ok(())
    // shell::repl()
}
