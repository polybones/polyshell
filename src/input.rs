use std::io::{StdoutLock, Write, stdin, stdout};

use anyhow::{Result, anyhow};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{style, terminal_size};
use crate::shell::{self, Shell};

const DEFAULT_PROMPT: &'static str = "~> ";

pub struct Reader<'a> {
    shell: Shell,
    term: RawTerminal<StdoutLock<'a>>,
    buffer: String,
}

impl<'a> Reader<'a> {
    pub fn new() -> Result<Self> {
        let shell = Shell::default();
        let term = stdout()
            .lock()
            .into_raw_mode()?;
        let buffer = String::new();
        Ok(Self {
            shell,
            term,
            buffer,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        write!(self.term, "{DEFAULT_PROMPT}")?;
        self.term.flush()?;
        
        for key in stdin().keys() {
            if let Ok(k) = key {
                match k {
                    Key::Char('\n') => {
                        let cols = terminal_size().unwrap().0;
                        if self.buffer.len() > 0 {
                            self.term.suspend_raw_mode().ok();
                            print!("\r\n");
                            if let Err(err) = shell::run(&self.buffer, &mut self.shell) {
                                print!("{err}\n");
                            }
                            self.buffer.clear();
                            write!(
                                self.term,
                                "{}${}{}\r",
                                style::Invert,
                                style::Reset,
                                " ".repeat(cols as usize - 1),
                            )?;
                        }
                        else {
                            write!(self.term, "{}\r", " ".repeat(cols as usize))?;
                        }
                        write!(self.term, "{DEFAULT_PROMPT}")?;
                        self.term.flush()?;
                        self.buffer.clear();
                        self.term.activate_raw_mode()?;
                    },
                    Key::Char(c) => {
                        self.buffer.push(c);
                        write!(self.term, "{c}")?;
                        self.term.flush()?;
                    },
                    Key::Backspace => todo!(),
                    Key::Left => todo!(),
                    Key::Right => todo!(),
                    _ => {},
                }
            }
            else {
                return Err(anyhow!("{}", key.unwrap_err()));
            }
        }
        Ok(())
    }
}
