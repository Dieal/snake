use crate::{Position, screen::Screen};

pub struct Drawer;

impl Drawer {
    pub fn draw_rectangle (screen: &mut Screen, start: Position, width: u16, height: u16) {
        let cursor = &mut screen.cursor;
        if width == 0 || height == 0 {
            return;
        }

        let top_line = start.line;
        let bottom_line = top_line + height;
        let left_column = start.column;
        let right_column = left_column + width;
        cursor.jump(top_line, left_column);

        // Renders top line
        for i in 1..=width {
            match i {
                1 => print!("╭"),
                num if num == width => print!("╮"),
                _ => print!("─"),
            }
        }
        cursor.down(1);

        // Renders left and side lines
        for _ in 1..height-1 {
            cursor.jump_to_col(left_column);
            print!("│");
            cursor.jump_to_col(right_column);
            print!("│");
            cursor.down(1);
        }

        // Renders bottom line
        cursor.jump_to_col(left_column);
        for i in 1..=width {
            match i {
                1 => print!("╰"),
                num if num == width => print!("╯"),
                _ => print!("─"),
            }
        }
    }

}
