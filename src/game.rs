use std::{thread::sleep, time::Duration};
use crossterm::event::{Event, KeyCode};
use log::{debug, info};
use rand::Rng;
use crate::{Border, Direction, Position};
use crate::screen::Screen;
use crate::drawing::Drawer;
use crate::snake::{Snake, SnakeNode};

fn random_position(border: &Border) -> Position {
    let mut rng = rand::rng();
    let border: Border = *border;
    Position::new(
        rng.random_range(border.start_line + 2..border.end_line-1), 
        rng.random_range(border.start_col + 2..border.end_col-1)
    )
}

pub enum MapItemType {
    Food,
    Hazard,
}

#[allow(dead_code)]
pub struct MapItem {
    pub item_type: MapItemType,
    pub position: Position,
}

impl MapItem {
    pub fn new(item_type: MapItemType, position: Position) -> Self {
        MapItem {
            item_type,
            position,
        }
    }

    fn set_random_position(&mut self, border: &Border, invalid_positions: &Vec<Position>) {
        let mut position = random_position(border);

        let mut invalid: bool = true;
        while invalid {
            invalid = false;
            for invalid_position in invalid_positions {
                if position == *invalid_position {
                    position = random_position(border);
                    invalid = true;
                }
            }
        }
        self.position = position;
    }

}

impl Default for MapItem {
    fn default() -> Self {
        Self::new(MapItemType::Food, Position::default())
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct SnakeGame {
    screen: Screen,
    score: u16,
    food: MapItem,
    hazards: Vec<MapItem>,
    hazards_count: u8,
    snake: Snake,
    border: Border,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut screen: Screen = Screen::new();
        screen.init();
        screen.hide_cursor();

        let (width, height) = screen.get_terminal_size();
        info!("[Screen] Width: {width}, Height: {height}");
        SnakeGame {
            screen,
            score: 0,
            food: MapItem::new(MapItemType::Food, Position::default()),
            hazards: Vec::new(),
            hazards_count: 20,
            snake: Snake::default(),
            border: Border::new(10, width - 10, 4, height - 4),
        }
    }

    pub fn init(&mut self) {
        let screen = &mut self.screen;
        let (width, height) = screen.get_terminal_size();
        let border: Border = self.border;
        info!("[Border]\n{:#?}", border);
        Drawer::draw_borders(screen, &border);
        Drawer::draw_text(screen, format!("Score: {}", self.score).as_str(), Position::new(border.start_line - 1, border.start_col + 2));

        let snake_boundaries: Border = Border::new(
            border.start_col + 2, 
            border.end_col - 1, 
            border.start_line + 1, 
            border.end_line - 1
        );
        let head_position = random_position(&border);
        self.snake = Snake::new(
            Direction::Up, 
            SnakeNode::new(
                head_position
            ),
            Some(snake_boundaries)
        );
        self.snake.add_tails(3);
        self.food.set_random_position(&border, &self.snake.get_positions());

        // ==== DRAWING ==== //
        Drawer::render_map_item(&mut self.screen, &self.food);
        Drawer::draw_snake(&mut self.screen, &self.snake);
        Screen::flush();

        // Add hazards
        for _ in 1..=self.hazards_count {
            let mut hazard = MapItem::new(MapItemType::Hazard, Position::default());
            hazard.set_random_position(&border, &self.snake.get_positions());
            Drawer::render_map_item(&mut self.screen, &hazard);
            self.hazards.push(hazard);
        }
    }

    pub fn run(&mut self) {
        self.init();
        let border: Border = self.border;
        let mut game_lost = false;
        while !game_lost {
            // Handles input and exit if necessary
            if let Ok(should_exit) = self.handle_input() {
                if should_exit {
                    break;
                }
            }
            sleep(Duration::from_millis(100)); // TODO: make clock speed configurable

            Drawer::delete_snake(&mut self.screen, &self.snake); // Delete previous snake
            self.snake.update_positions();
            Drawer::draw_snake(&mut self.screen, &self.snake); // Draws new snake

            game_lost = self.snake.is_eating_tail();
            let head_position = *self
                .snake
                .get_head()
                .get_position();
            if self.food.position == head_position {
                info!(
                    "Ate food at line {} and column {}",
                    self.food.position.line, self.food.position.column
                );
                self.score += 1;
                self.snake.add_tail();
                self.food.set_random_position(&border, &self.snake.get_positions());
                Drawer::render_map_item(
                    &mut self.screen, 
                    &self.food
                );
                Drawer::draw_text(&mut self.screen, format!("Score: {}", self.score).as_str(), Position::new(self.border.start_line - 1, self.border.start_col + 2));
            }

            for hazard in self.hazards.iter() {
                if hazard.position == head_position {
                    game_lost = true;
                    break;
                }
            }

            Screen::flush();
        }
        Screen::erase_screen();
        
        if game_lost {
            print!("You lost the game");
        }
    }

    fn handle_input(&mut self) -> Result<bool, std::io::Error> {
        let mut should_exit: bool = false;
        let event_available = Screen::poll_event()?;
        if event_available {
            let event = Screen::get_event()?;
            debug!("Event available, {:#?}", event);
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                        if self.snake.can_go_in_direction(Direction::Left) {
                            self.snake.change_direction(Direction::Left)
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                        if self.snake.can_go_in_direction(Direction::Right) {
                            self.snake.change_direction(Direction::Right)
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        if self.snake.can_go_in_direction(Direction::Up) {
                            self.snake.change_direction(Direction::Up)
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        if self.snake.can_go_in_direction(Direction::Down) {
                            self.snake.change_direction(Direction::Down)
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => should_exit = true,
                    _ => (),
                }
            }

            // Empties the queue except for the last one
            debug!("Emptied the event queue");
            while Screen::poll_event()? {
                let _ = Screen::get_event();
            }
        }
        Ok(should_exit)
    }
}
