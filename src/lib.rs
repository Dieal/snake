use log::info;

pub mod game;
pub mod menu;
mod cursor;
mod drawing;
mod screen;
mod snake;

const ESC: &str = "\x1b";
const WHITE: u16 = 15;
const GREEN: u16 = 2;
const BLUE: u16 = 4;
const DARK_BLUE: u16 = 18;
const RED: u16 = 1;
type Line = u16;
type Column = u16;
type Height = u16;
type Width = u16;

#[derive(Default, Debug, Clone, Copy)]
pub struct Border {
    pub start_col: Column,
    pub end_col: Column,
    pub start_line: Line,
    pub end_line: Line,
}

#[allow(dead_code)]
impl Border {
    pub fn new(start_col: Column, end_col: Column, start_line: Line, end_line: Line) -> Self {
        Border { start_col, start_line, end_col, end_line }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Position {
    pub line: u16,
    pub column: u16,
    pub boundaries: Option<Border>,
}

#[allow(dead_code)]
impl Position {
    pub fn new(line: Line, column: Column) -> Self {
        Position { line, column, boundaries: None }
    }

    pub fn set_line(&mut self, line: Line) {
        self.line = line;
    }

    pub fn set_column(&mut self, column: Column) {
        self.column = column;
    }

    pub fn set_boundaries(&mut self, boundaries: Border) {
        self.boundaries = Some(boundaries);
    }

    pub fn increment_col(&mut self, offset: Column) {
        let mut new_column = self.column + offset;
        if let Some(boundaries) = self.boundaries {
            if new_column >= boundaries.end_col {
                info!("[Increasing column] New Column: {new_column}, End Column: {}", boundaries.end_col);
                new_column = boundaries.start_col;
            }
        }
        self.set_column(new_column);
    }

    pub fn decrement_col(&mut self, offset: Column) {
        let mut new_column = self.column.saturating_sub(offset);
        if let Some(boundaries) = self.boundaries {
            if new_column < boundaries.start_col {
                new_column = boundaries.end_col;
            }
        }
        self.set_column(new_column);
    }

    pub fn increment_line(&mut self, offset: Column) {
        let mut new_line = self.line + offset;
        if let Some(boundaries) = self.boundaries {
            if new_line > boundaries.end_line {
                new_line = boundaries.start_line;
            }
        }
        self.set_line(new_line);
    }

    pub fn decrement_line(&mut self, offset: Column) {
        let mut new_line = self.line.saturating_sub(offset);
        if let Some(boundaries) = self.boundaries {
            if new_line < boundaries.start_line {
                new_line = boundaries.end_line;
            }
        }
        self.set_line(new_line);
    }

    pub fn equals(pos1: &Self, pos2: &Self) -> bool {
        pos1.line == pos2.line && pos1.column == pos2.column
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        Self::equals(self, other)  
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
