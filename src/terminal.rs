use super::Position;

use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    #[allow(clippy::missing_errors_doc)]
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    #[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
    pub fn cursor_position(position: &Position) {
        let Position{ mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;

        print!("{}", termion::cursor::Goto(x, y));
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn invert_style() {
        print!("{}", termion::style::Invert);
    }

    pub fn reset_style() {
        print!("{}", termion::style::Reset);
    }
}