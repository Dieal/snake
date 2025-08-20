mod screen;
mod cursor;
mod drawing;

use std::{thread::sleep, time::Duration};

use crossterm::event::{Event, KeyCode};
use drawing::Drawer;
use rand::Rng;
use screen::Screen;

const ESC: &str = "\x1b";
type Line = u16;
type Column = u16;
type Height = u16;
type Width = u16;

#[derive(Default, Debug)]
pub struct Position {
    pub line: u16,
    pub column: u16,
}

#[allow(dead_code)]
impl Position {
    pub fn new(line: u16, column: u16) -> Self {
        Position { line, column }
    }
}

#[derive(Debug)]
pub struct Snake {
}

#[derive(Debug, Default)]
pub struct SnakeGame {
    screen: Screen,
    score: u16,
    food_position: Position,
}

impl SnakeGame {
    pub fn new() -> Self {
        SnakeGame {
            screen: Screen::new(),
            score: 0,
            food_position: Position::default(),
        }
    }

    pub fn run(&mut self) {
        let screen = &mut self.screen;
        screen.init();
        screen.hide_cursor();

        let (width, height) = screen.get_terminal_size();

        // Draw screen borders
        Drawer::draw_rectangle(screen, Position { line: 1, column: 0}, width, height);

        self.random_food_position();
        self.render_food();
        Screen::flush();

        loop {
            // Handles input and exit if necessary
            if let Ok(should_exit) = self.handle_input() {
                if should_exit {
                    break;
                }
            }

            sleep(Duration::from_millis(300));
            Screen::flush();
        }
        Screen::erase_screen();
    } 

    fn handle_input(&mut self) -> Result<bool, std::io::Error> {
        let mut should_exit: bool = false;
        let cursor = &mut self.screen.cursor;
        let event_available = Screen::poll_event().expect("Expected boolean");
        if event_available {
            let event = Screen::get_event()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => cursor.left(1),
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => cursor.right(1),
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => cursor.up(1),
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => cursor.down(1),
                    KeyCode::Esc | KeyCode::Char('q') => should_exit = true,
                    _ => (),
                }
            }
        }
        Ok(should_exit)
    }

    fn random_food_position(&mut self) {
        let mut rng = rand::rng();
        let (width, height) = self.screen.get_terminal_size();
        self.food_position = Position::new(rng.random_range(1..height), rng.random_range(1..width));
    }

    fn render_food(&mut self) {
        Screen::draw(&mut self.screen, self.food_position.line, self.food_position.column, '🍎');
    }
}
