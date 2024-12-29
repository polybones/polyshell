mod config;
mod lang;
mod process;
mod shell;

use anyhow::Result;

fn main() -> Result<()> {
    shell::repl()
}
