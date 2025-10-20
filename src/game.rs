use std::{thread::sleep, time::Duration};
use crossterm::event::{Event, KeyCode};
use log::info;
use rand::Rng;
use crate::{drawing, Direction, Position};
use crate::screen::Screen;
use crate::drawing::Drawer;
use crate::snake::{Snake, SnakeNode};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SnakeGame {
    screen: Screen,
    score: u16,
    food_position: Position,
    snake: Snake,
}

impl SnakeGame {
    pub fn new() -> Self {
        SnakeGame {
            screen: Screen::new(),
            score: 0,
            food_position: Position::default(),
            snake: Snake::default(),
        }
    }

    pub fn run(&mut self) {
        let screen = &mut self.screen;
        screen.init();
        screen.hide_cursor();

        let (width, height) = screen.get_terminal_size();
        Drawer::draw_rectangle(screen, Position { line: 2, column: 0 }, width, height);
        Drawer::draw_text(screen, format!("Score: {}", self.score).as_str(), Position { line: 1, column: 2});

        let head_position = self.random_position();
        self.snake = Snake::new(Direction::Up, SnakeNode::new(
            head_position
        ));
        self.snake.add_tails(3);
        self.set_random_food_position(&head_position);

        // ==== DRAWING ==== //
        Drawer::render_food(&mut self.screen, &self.food_position);
        Drawer::draw_snake(&mut self.screen, &self.snake);
        Screen::flush();

        loop {
            // Handles input and exit if necessary
            if let Ok(should_exit) = self.handle_input() {
                if should_exit {
                    break;
                }
            }
            sleep(Duration::from_millis(50)); // TODO: make clock speed configurable

            Drawer::delete_snake(&mut self.screen, &self.snake); // Delete previous snake
            self.snake.update_positions();
            Drawer::draw_snake(&mut self.screen, &self.snake); // Draws new snake

            let head_position = *self
                .snake
                .get_head()
                .expect("Should have a head")
                .get_position();
            if self.food_position == head_position {
                info!(
                    "Ate food at line {} and column {}",
                    self.food_position.line, self.food_position.column
                );
                self.score += 1;
                self.snake.add_tail();
                self.set_random_food_position(&head_position);
                Drawer::render_food(
                    &mut self.screen, 
                    &self.food_position
                );
                Drawer::draw_text(&mut self.screen, format!("Score: {}", self.score).as_str(), Position { line: 1, column: 2});
            }

            Screen::flush();
        }
        Screen::erase_screen();
    }

    fn handle_input(&mut self) -> Result<bool, std::io::Error> {
        let mut should_exit: bool = false;
        let event_available = Screen::poll_event().expect("Expected boolean");
        if event_available {
            let event = Screen::get_event()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                        self.snake.change_direction(Direction::Left)
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                        self.snake.change_direction(Direction::Right)
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        self.snake.change_direction(Direction::Up)
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        self.snake.change_direction(Direction::Down)
                    }
                    KeyCode::Esc | KeyCode::Char('q') => should_exit = true,
                    _ => (),
                }
            }
        }
        Ok(should_exit)
    }

    // Invalid position is the snake's head position, so that the food
    // doesn't get generated on top of the snake's head
    fn set_random_food_position(&mut self, invalid_position: &Position) {
        let mut position = self.random_position();
        while position == *invalid_position {
            position = self.random_position();
        }

        self.food_position = position;
    }

    fn random_position(&mut self) -> Position {
        let mut rng = rand::rng();
        let (width, height) = self.screen.get_terminal_size();
        Position::new(rng.random_range(1..height-1), rng.random_range(1..width-1))
    }
}
