use log::info;

use crate::{game::{Object, ObjectType}, screen::Screen, snake::Snake, Border, Column, Line, Position, GREEN, RED};

pub struct Drawer;
impl Drawer {
    fn eat_cell(screen: &mut Screen, line: Line, column: Column) {
        screen.draw(line, column, ' ');
    }

    pub fn draw_text(screen: &mut Screen, text: &str, position: Position) {
        let mut column = position.column;
        for char in text.chars() {
            screen.draw(position.line, column, char);
            column += 1;
        }
    }

    pub fn delete_snake(screen: &mut Screen, snake: &Snake) {
        info!("======== Start Deleting snake ======");
        for node in snake.get_list() {
            let position = node.get_position();
            Self::eat_cell(screen, position.line, position.column);
            info!("Deleted at {:?}", position);
        }
        info!("======== End Deleting snake ======");
    }

    pub fn render_object (screen: &mut Screen, object: &Object) {
        let (icon, color) = match object.object_type {
            ObjectType::Food => ('✿', 26),
            ObjectType::Hazard => ('☠', RED),
        };

        Screen::draw_colored(
            screen,
            object.position.line,
            object.position.column,
            icon,
            color
        );
    }

    pub fn draw_snake(screen: &mut Screen, snake: &Snake) {
        info!("======== Start Drawing snake ======");
        let mut iterator = snake.get_list().iter();
        let head = iterator.next().expect("Should have head").get_position();
        screen.draw_colored(head.line, head.column, '◉', GREEN);

        for node in iterator {
            let position = node.get_position();
            screen.draw_colored(position.line, position.column, '⬤', GREEN);
            info!("Drawed at {:?}", position);
        }
        info!("======== End Drawing snake ======");
    }

    pub fn draw_borders(screen: &mut Screen, border: &Border) {
        Self::draw_rectangle(
            screen, 
            Position::new(
                border.start_line, 
                border.start_col
            ), 
            border.end_col - border.start_col,
            border.end_line - border.start_line
        );
    }

    pub fn draw_rectangle(screen: &mut Screen, start: Position, width: u16, height: u16) {
        let cursor = &mut screen.cursor;
        if width == 0 || height == 0 {
            return;
        }

        let start_line = start.line;
        let end_line = start_line + height;
        let start_col = start.column;
        let end_col = start_col + width;
        cursor.jump(start_line, start_col);

        // Renders top line
        for i in start_col..=end_col {
            match i {
                start if start == start_col => print!("╭"),
                num if num == end_col => print!("╮"),
                _ => print!("─"),
            }
        }
        cursor.down(1);

        // Renders left and side lines
        for _ in start_line..end_line - 1 {
            cursor.jump_to_col(start_col);
            print!("│");
            cursor.jump_to_col(end_col);
            print!("│");
            cursor.down(1);
        }

        // Renders bottom line
        cursor.jump_to_col(start_col);
        for i in start_col..=end_col {
            match i {
                start if start == start_col => print!("╰"),
                num if num == end_col => print!("╯"),
                _ => print!("─"),
            }
        }
    }
}
