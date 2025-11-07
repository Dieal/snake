use std::iter;

use log::info;

use crate::{game::{MapItem, MapItemType}, screen::Screen, snake::Snake, Border, Column, Line, Direction, Position, GREEN, RED};

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

    pub fn render_map_item (screen: &mut Screen, item: &MapItem) {
        let (icon, color) = match item.item_type {
            MapItemType::Food => ('✿', 26),
            MapItemType::Hazard => ('☠', RED),
        };

        Screen::draw_colored(
            screen,
            item.position.line,
            item.position.column,
            icon,
            color
        );
    }

    pub fn draw_snake(screen: &mut Screen, snake: &Snake) {
        info!("======== Start Drawing snake ======");
        let head = snake.get_head();
        let head_position = head.get_position();
        let mut iterator = snake.get_list().iter().peekable();
        let _ = iterator.next(); // Skips head
        screen.draw_colored(head_position.line, head_position.column, '◉', GREEN);

        while let Some(node) = iterator.next() {
            let position = node.get_position();
            if let Some(direction) = node.get_direction() {
                let mut character = match direction {
                    Direction::Right | Direction::Left => '━',
                    Direction::Up | Direction::Down => '┃',
                };

                // I honestly don't understand why this works. The characters don't match with the directions, but
                // somehow they are correctly rendered
                if let Some(next_node) = iterator.peek() {
                    if let Some(following_direction) = next_node.get_direction() {
                        if direction != following_direction {
                            if (matches!(direction, Direction::Down) && matches!(following_direction, Direction::Left))
                                || (matches!(direction, Direction::Right) && matches!(following_direction, Direction::Up)){
                                    character = '┏';
                                } else if (matches!(direction, Direction::Down) && matches!(following_direction, Direction::Right))
                                    || (matches!(direction, Direction::Left) && matches!(following_direction, Direction::Up)) {
                                        character = '┓';
                                    } else if (matches!(direction, Direction::Up) && matches!(following_direction, Direction::Left))
                                        || (matches!(direction, Direction::Right) && matches!(following_direction, Direction::Down)) {
                                            character = '┗';
                                        } else if (matches!(direction, Direction::Up) && matches!(following_direction, Direction::Right))
                                            || (matches!(direction, Direction::Left) && matches!(following_direction, Direction::Down)){
                                                character = '┛';
                                        }
                                        info!("[NODE DIRECTION]: {:?}, [FOLLOWING NODE DIRECTION]: {:?}, [CHARACTER]: {character}", direction, following_direction);
                        }
                    }
                    screen.draw_colored(position.line, position.column, character, GREEN);
                }
            } else {
                screen.draw_colored(position.line, position.column, '⬤', GREEN);
            }
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
