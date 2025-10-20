pub mod game;
mod cursor;
mod drawing;
mod screen;
mod snake;

const ESC: &str = "\x1b";
type Line = u16;
type Column = u16;
type Height = u16;
type Width = u16;

#[derive(Default, Debug, Clone, Copy)]
pub struct Position {
    pub line: u16,
    pub column: u16,
}

#[allow(dead_code)]
impl Position {
    pub fn new(line: Line, column: Column) -> Self {
        Position { line, column }
    }

    pub fn set_line(&mut self, line: Line) {
        self.line = line;
    }

    pub fn set_column(&mut self, column: Column) {
        self.column = column;
    }

    pub fn increment_col(&mut self, offset: Column) {
        self.set_column(self.column + offset);
    }

    pub fn decrement_col(&mut self, offset: Column) {
        if offset < self.column {
            self.set_column(self.column - offset);
        }
    }

    pub fn increment_line(&mut self, offset: Column) {
        self.set_line(self.line + offset);
    }

    pub fn decrement_line(&mut self, offset: Column) {
        if offset < self.line {
            self.set_line(self.line - offset);
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
