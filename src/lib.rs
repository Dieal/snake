mod cursor;
mod drawing;
mod screen;
mod snake;

use snake::{Snake, SnakeNode};
use std::{fmt::Display, thread::sleep, time::Duration};

use crossterm::event::{Event, KeyCode};
use drawing::Drawer;
use log::info;
use rand::Rng;
use screen::Screen;

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

        // Draw screen borders
        Drawer::draw_rectangle(screen, Position { line: 1, column: 0 }, width, height);

        self.snake = Snake::new(Direction::Up, SnakeNode::new(Position::new(10, 10)));
        self.random_food_position();
        while self.food_position
            == *self
                .snake
                .get_head()
                .expect("Should have a head")
                .get_position()
        {
            self.random_food_position();
        }
        self.render_food();
        Drawer::draw_snake(&mut self.screen, &self.snake);
        Screen::flush();

        loop {
            // Handles input and exit if necessary
            if let Ok(should_exit) = self.handle_input() {
                if should_exit {
                    break;
                }
            }
            sleep(Duration::from_millis(100));

            Drawer::delete_snake(&mut self.screen, &self.snake); // Delete previous snake
            self.snake.update_positions();

            Drawer::draw_snake(&mut self.screen, &self.snake); // Draws new snake

            if self.food_position
                == *self
                    .snake
                    .get_head()
                    .expect("Should have a head")
                    .get_position()
            {
                info!(
                    "Ate food at line {} and column {}",
                    self.food_position.line, self.food_position.column
                );
                self.score += 1;
                self.snake.add_tail();
                self.random_food_position();
                self.render_food();
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

    fn random_food_position(&mut self) {
        self.food_position = self.random_position();
    }

    fn random_position(&mut self) -> Position {
        let mut rng = rand::rng();
        let (width, height) = self.screen.get_terminal_size();
        Position::new(rng.random_range(1..height), rng.random_range(1..width))
    }

    fn render_food(&mut self) {
        Screen::draw(
            &mut self.screen,
            self.food_position.line,
            self.food_position.column,
            '🍎',
        );
    }
}
