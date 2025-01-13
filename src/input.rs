use std::io::{StdoutLock, Write, stdin, stdout};

use anyhow::{Result, anyhow};
use termion::clear;
use termion::cursor;
use termion::cursor::DetectCursorPos;
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
    cursor: usize,
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
            cursor: 0,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        write!(self.term, "{DEFAULT_PROMPT}{}", cursor::SteadyBar)?;
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
                                print!("polyshell: {err}\n");
                            }
                            self.buffer.clear();
                            self.cursor = 0;
                        }
                        write!(
                            self.term,
                            "{}${}{}\r",
                            style::Invert,
                            style::Reset,
                            " ".repeat(cols as usize - 1),
                        )?;
                        self.term.activate_raw_mode()?;
                        write!(self.term, "{DEFAULT_PROMPT}")?;
                        self.term.flush()?;
                        self.buffer.clear();
                    },
                    Key::Ctrl('w') => todo!(),
                    Key::Char(c) => {
                        self.buffer.insert(self.cursor, c);
                        self.cursor += 1;
                        self.render_user_input()?;
                    },
                    Key::Backspace if self.cursor > 0 => {
                        self.cursor -= 1;
                        self.buffer.remove(self.cursor);
                        self.render_user_input()?;
                    },
                    Key::Left if self.cursor > 0 => {
                        self.cursor -= 1;
                        write!(self.term, "{}", cursor::Left(1))?;
                        self.term.flush()?;
                    },
                    Key::Right if self.cursor < self.buffer.len() => {
                        self.cursor += 1;
                        write!(self.term, "{}", cursor::Right(1))?;
                        self.term.flush()?;
                    },
                    _ => {},
                }
            }
            else {
                return Err(anyhow!("{}", key.unwrap_err()));
            }
        }
        Ok(())
    }

    fn render_user_input(&mut self) -> Result<()> {
        // TODO rewrite messy function.
        let mx = DEFAULT_PROMPT.len() as u16 + 1;
        let cops = self.term.cursor_pos().unwrap();
        let x = std::cmp::max(mx, mx + self.cursor as u16);
        write!(
            self.term,
            "{}{}{}{}{}{}",
            cursor::Hide,
            cursor::Goto(mx, cops.1),
            clear::AfterCursor,
            self.buffer,
            cursor::Show,
            cursor::Goto(x, cops.1),
        )?;
        self.term.flush()?;
        Ok(())
    }
}
