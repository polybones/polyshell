mod lang;
mod proc;
mod read;

use anyhow::Result;

fn main() -> Result<()> {
    // let mut lexer = lang::lexer::Lexer::new(&stress_test);
    // println!("{:#?}", lexer.get_tokens());
    // Ok(())
    read::repl()
}
