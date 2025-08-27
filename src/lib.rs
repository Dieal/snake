mod cursor;
mod drawing;
mod screen;

use std::{thread::sleep, time::Duration};

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

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    prev_direction: Direction,
    positions: Vec<Position>, // First element is the head
}

impl Snake {
    pub fn new() -> Self {
        Self::from(Direction::Up, vec![Position { line: 1, column: 1 }])
    }

    pub fn from(direction: Direction, positions: Vec<Position>) -> Self {
        Snake {
            direction,
            prev_direction: Direction::Up,
            positions,
        }
    }

    pub fn update_positions(&mut self) {
        let len = self.positions.len();
        let head = *self.get_head().unwrap();
        info!("Length: {len}");
        for position in self.positions.iter_mut() {
            info!(
                "Before... Line: {}, Column: {}",
                position.line, position.column
            );
            match self.direction {
                Direction::Up => position.decrement_line(1),
                Direction::Down => {
                    if len == 1 {
                        position.line += 1;
                        continue;
                    }

                    match self.prev_direction {
                        Direction::Left | Direction::Right | Direction::Down => position.line += 1,
                        _ => (),
                    }
                }
                Direction::Left => {
                    if let Direction::Left = self.prev_direction {
                        continue;
                    }

                    if *position == head {
                        position.decrement_col(1);
                        continue;
                    }
                    position.increment_line(1);
                }
                Direction::Right => {
                    if let Direction::Right = self.prev_direction {
                        position.increment_col(1);
                        continue;
                    }

                    if *position == head {
                        position.increment_col(1);
                        continue;
                    } else {
                        position.decrement_line(1);
                    }
                }
            }
            info!(
                "After... Line: {}, Column: {}",
                position.line, position.column
            );
        }
    }

    pub fn get_head(&self) -> Option<&Position> {
        self.positions.first()
    }

    pub fn get_tail(&self) -> Option<&Position> {
        self.positions.last()
    }

    pub fn get_positions(&self) -> &Vec<Position> {
        &self.positions
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn add_tail(&mut self) {
        if let Some(tail) = self.get_tail() {
            let position: Position = match self.direction {
                Direction::Up => Position {
                    line: tail.line + 1,
                    column: tail.column,
                },
                Direction::Down => Position {
                    line: tail.line - 1,
                    column: tail.column,
                },
                Direction::Right => Position {
                    line: tail.line,
                    column: tail.column - 1,
                },
                Direction::Left => Position {
                    line: tail.line,
                    column: tail.column + 1,
                },
            };
            self.positions.push(position);
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.prev_direction = self.direction;
        self.direction = direction;
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
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

        self.snake = Snake::from(Direction::Up, vec![self.random_position()]);
        self.random_food_position();
        while self.food_position == *self.snake.get_head().expect("Should have a head") {
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

            if self.food_position == *self.snake.get_head().expect("Should have a head") {
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
