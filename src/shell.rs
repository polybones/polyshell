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

use crate::lang::*;

#[inline(always)]
pub fn repl() -> Result<()> {
    let bump: Bump = Bump::new();
    let mut ctx = evaluator::Context::default();
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
                if !buffer.is_empty() {
                    let res: Result<()> = (|| {
                        let tks = lexer::Lexer::new(&buffer).tokenize()?;
                        let ast = parser::parse(&bump, tks)?;
                        evaluator::eval_program(ast, &mut ctx)?;
                        Ok(())
                    })();
                    if let Err(err) = res {
                        println!("polyshell: {err}");
                    }
                }
                stdout.activate_raw_mode()?;

                // Reset cursor position
                let new_pos = stdout.cursor_pos()?;
                if new_pos.0 > 1 {
                    // Handle 'partial lines'
                    write!(stdout, "{}", cursor::Goto(1, new_pos.1+1))?;
                }
                else {
                    write!(stdout, "{}", cursor::Goto(1, new_pos.1))?;
                }
                buffer.clear();
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
