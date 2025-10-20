use log::info;

use crate::{screen::Screen, Column, Line, Position, snake::Snake};

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

    pub fn render_food(screen: &mut Screen, food_position: &Position) {
        Screen::draw(
            screen,
            food_position.line,
            food_position.column,
            '✿',
        );
    }

    pub fn draw_snake(screen: &mut Screen, snake: &Snake) {
        info!("======== Start Drawing snake ======");
        let mut iterator = snake.get_list().iter();
        let head = iterator.next().expect("Should have head").get_position();
        screen.draw(head.line, head.column, '◉');

        for node in iterator {
            let position = node.get_position();
            screen.draw(position.line, position.column, '⬤');
            info!("Drawed at {:?}", position);
        }
        info!("======== End Drawing snake ======");
    }

    pub fn draw_rectangle(screen: &mut Screen, start: Position, width: u16, height: u16) {
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
        for _ in 1..height - 1 {
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
