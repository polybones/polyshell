mod config;
mod lang;
mod process;
mod shell;

use anyhow::Result;

fn main() -> Result<()> {
    // let bump = bumpalo::Bump::new();
    // let mut ctx = lang::evaluator::Context::default();
    // let now = std::time::Instant::now();
    // let mut lexer = lang::lexer::Lexer::new("one two three four five");
    // let _ = lexer.tokenize();
    // let end = std::time::Instant::now();
    // println!("took {:?}", end - now);
    // Ok(())
    shell::repl()
}
