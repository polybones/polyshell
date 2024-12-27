use std::io::{stdin, stdout, Write};

use anyhow::Result;
use bumpalo::Bump;
use termion::clear;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::scroll;
use termion::terminal_size;

use crate::lang::{lexer, parser};

#[inline(always)]
pub fn repl() -> Result<()> {
    let bump: Bump = Bump::new();
    let stdin = stdin();
    let mut stdout = stdout()
        .lock()
        .into_raw_mode()?;
    let mut buffer = String::new();
    let min_x: usize = 0; // TODO will be used for shell prompt

    for key in stdin.keys() {
        let cursor_pos = stdout.cursor_pos()?;

        match key? {
            Key::Char('\n') => {
                let term_size = terminal_size()?;

                // Move cursor down
                write!(stdout, "{}", cursor::Goto(1, cursor_pos.1))?;
                if cursor_pos.1 == term_size.1 {
                    write!(stdout, "{}", scroll::Up(1))?;
                }
                else {
                    write!(stdout, "{}", cursor::Down(1))?;
                }
                stdout.flush()?;
                
                stdout.suspend_raw_mode()?;
                
                let tokens = lexer::Lexer::new(&buffer).tokenize();
                let nodes = parser::parse(&bump, tokens);
                println!("{:#?}", nodes);
                // process::exec_external(&buffer);
                
                stdout.activate_raw_mode()?;
                buffer.clear();

                // Reset cursor position
                let cursor_pos = stdout.cursor_pos()?;
                write!(stdout, "{}", cursor::Goto(1, cursor_pos.1))?;
                stdout.flush()?;
            },
            Key::Char(c) => {
                buffer.push(c);
                write!(stdout, "{c}")?;
                stdout.flush()?;
            },
            Key::Backspace => {
                let cx = cursor_pos.0 as usize;
                if cx > min_x + 1 {
                    let index = (cx + min_x) - 2;
                    buffer.remove(index);
                    write!(stdout, "{}{}{}", cursor::Left(1), clear::AfterCursor, &buffer[index..])?;
                    stdout.flush()?;
                }
            },
            _ => {},
        }
    }
    
    Ok(())
}
