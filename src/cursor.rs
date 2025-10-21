use crate::{Column, Line, Position, ESC};

#[derive(Default, Debug)]
pub struct Cursor {
    position: Position,
    hidden: bool,
}

#[allow(dead_code)]
impl Cursor {
    pub fn new() -> Self {
        Cursor {
            position: Position::new(0, 0,),
            hidden: false,
        }
    }

    pub fn from(position: Position, hidden: bool) -> Self {
        Cursor { position, hidden }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn jump_to_line (&mut self, line: Line) {
        self.jump(line, self.position.column);
    }

    pub fn jump_to_col (&mut self, column: Column) {
        self.jump(self.position.line, column);
    }

    pub fn jump_home(&mut self) {
        self.jump(0, 0);
    }

    pub fn jump(&mut self, line: Line, column: Column) {
        print!("{ESC}[{line};{column}H");
        self.position.line = line;
        self.position.column = column;
    }

    pub fn jump_to_position(&mut self, position: Position) {
        self.jump(position.line, position.column);
    }

    pub fn up(&mut self, offset: u16) {
        self.move_cursor(CursorMovement::Up, offset);
    }

    pub fn down(&mut self, offset: u16) {
        self.move_cursor(CursorMovement::Down, offset);
    }

    pub fn left(&mut self, offset: u16) {
        self.move_cursor(CursorMovement::Left, offset);
    }

    pub fn right(&mut self, offset: u16) {
        self.move_cursor(CursorMovement::Right, offset);
    }

    pub fn move_cursor (&mut self, movement: CursorMovement, offset: u16) {
        print!("{ESC}[{offset}{}", movement.character());
        match movement {
            CursorMovement::Left => self.position.column -= if self.position.column > 0 { offset } else { 0 },
            CursorMovement::Right => self.position.column += offset,
            CursorMovement::Up => self.position.line -= if self.position.line > 0 { offset } else { 0 },
            CursorMovement::Down => self.position.line += offset
        }
    }

    pub fn hide(&mut self) {
        print!("{ESC}[?25l");
        self.hidden = true;
    }

    pub fn show(&mut self) {
        print!("{ESC}[?25h");
        self.hidden = false;
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
}

pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right
}

impl CursorMovement {
    pub fn character(&self) -> String {
        match self {
            CursorMovement::Up => String::from("A"),
            CursorMovement::Down => String::from("B"),
            CursorMovement::Left => String::from("D"),
            CursorMovement::Right => String::from("C")
        }
    }
}
