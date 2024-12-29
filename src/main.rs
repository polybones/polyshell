mod config;
mod lang;
mod process;
mod shell;

use anyhow::Result;

fn main() -> Result<()> {
    // let bump = bumpalo::Bump::new();
    // let mut ctx = lang::evaluator::Context::default();
    // let mut lexer = lang::lexer::Lexer::new("alias ls = eza --icons;ls");
    // let ast = lang::parser::parse(&bump, lexer.tokenize());
    // println!("ast: {ast:#?}");
    // lang::evaluator::eval_program(ast, &mut ctx);
    // println!("context: {ctx:#?}");
    // Ok(())
    shell::repl()
}
