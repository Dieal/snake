use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::terminal::enable_raw_mode;

use crate::cursor::{Cursor};
use crate::{Column, Height, Line, Position, Width, ESC};

#[derive(Default, Debug)]
pub struct Screen {
    pub cursor: Cursor,
    width: Line,
    height: Column,
}

#[allow(dead_code)]
impl Screen {
    pub fn new() -> Self {
        Screen { cursor: Cursor::new(), height: 0, width: 0 }
    }

    pub fn from(cursor_position: Position, height: Height, width: Width) -> Self {
        let cursor = Cursor::from(cursor_position, false);
        Screen { cursor, height, width }
    }

    pub fn init(&mut self) {
        let _ = enable_raw_mode(); // Hides input keys, input processed without enter, etc.

        Self::erase_screen();
        self.hide_cursor();
        self.update_terminal_size();
        Self::flush();
    }

    pub fn draw (&mut self, line: Line, column: Column, character: char) {
        self.cursor.jump(line, column);
        print!("{character}");
    }

    /// Returns width and height as tuple
    pub fn get_terminal_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn update_terminal_size(&mut self) {
        if let Some((width, height)) = terminal_size::terminal_size() {
            self.width = width.0;
            self.height = height.0;
        }
    }

    pub fn erase_screen() {
        print!("{ESC}[2J");
    }

    /// Returns true if an event is available to be read
    pub fn poll_event() -> std::io::Result<bool> {
        poll(Duration::from_millis(100))
    }

    /// Read event (if available) or block until available
    pub fn get_event() -> std::io::Result<Event> {
        read()
    }

    pub fn hide_cursor (&mut self) {
        self.cursor.hide();
    }

    pub fn show_cursor (&mut self) {
        self.cursor.show();
    }

    pub fn flush() {
        io::stdout().flush().unwrap();
    }
}
