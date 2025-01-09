mod input;
mod process;
mod shell;

use anyhow::Result;
use input::Reader;

fn main() -> Result<()> {
    let mut reader = Reader::new()?;
    reader.run();
    Ok(())
}
