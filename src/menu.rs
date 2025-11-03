use std::{collections::HashMap, option, thread::sleep, time::Duration};

use crossterm::event::{Event, KeyCode};
use log::debug;

use crate::{game::SnakeGame, screen::{self, Screen}, Position, ESC};

struct MenuOption {
    text: String,
    start_position: Position,
}

impl MenuOption {
    pub fn new(text: String) -> Self {
        MenuOption { text, start_position: Position::default() }
    }

    pub fn new_with_position(text: String, start: Position) -> Self {
        MenuOption { text, start_position: start }
    }

    pub fn set_position(&mut self, position: Position) {
        self.start_position = position;
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum MenuOptionType {
    Play,
    Options,
    Quit,
}

impl MenuOptionType {
    fn next(&self) -> Self {
        match self {
            Self::Play => Self::Options,
            Self::Options => Self::Quit,
            Self::Quit => Self::Play,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Play => Self::Quit,
            Self::Options => Self::Play,
            Self::Quit => Self::Options,
        }
    }
}

pub struct Menu {
    screen: Screen,
    options: HashMap<MenuOptionType, MenuOption>,
    selected_option: MenuOptionType,
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Menu {
    pub fn new() -> Self {
        let mut screen = Screen::new();
        screen.init();

        let mut options: HashMap<MenuOptionType, MenuOption> = HashMap::new();
        options.insert(MenuOptionType::Play, MenuOption::new(String::from("New Game")));
        options.insert(MenuOptionType::Options, MenuOption::new(String::from("Options")));
        options.insert(MenuOptionType::Quit, MenuOption::new(String::from("Quit")));

        Menu { screen, options, selected_option: MenuOptionType::Play }
    }

    pub fn run(&mut self) {
        self.draw();

        while let Ok(Event::Key(key)) = Screen::get_event() {
            debug!("Found key {}", key.code);
            // Resets previous selected option formatting
            if let Some(option) = self.options.get(&self.selected_option) {
                let pos = option.start_position;
                self.screen.draw_formatted_text(pos.line, pos.column, option.text.as_str(), 0);
            }

            match key.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    match self.selected_option {
                        MenuOptionType::Play => {
                            let mut game = SnakeGame::new();
                            game.run();

                            // After the game ends, the menu re-renders
                            self.draw();
                            // TODO Game over menu
                        },
                        MenuOptionType::Options => {},
                        MenuOptionType::Quit => break,
                    }
                },
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                    self.selected_option = self.selected_option.prev();
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                    self.selected_option = self.selected_option.next();
                }
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            } 

            if let Some(option) = self.options.get(&self.selected_option) {
                self.screen.cursor.jump_to_position(option.start_position);
                print!("{ESC}[4m{}", option.text);
                print!("{ESC}[24m"); // Reset underline
            }
            Screen::flush();
        }

        Screen::erase_screen();
        Screen::flush();
    }

    pub fn draw(&mut self) {
        let (width, _) = self.screen.get_terminal_size();
        let cursor = &mut self.screen.cursor;
        let text = indoc::indoc! {
            r"                                                        
              .--.--.                                ,-.            
             /  /    '.                          ,--/ /|            
            |  :  /`. /      ,---,             ,--. :/ |            
            ;  |  |--`   ,-+-. /  |            :  : ' /             
            |  :  ;_    ,--.'|'   |  ,--.--.   |  '  /      ,---.   
             \  \    `.|   |  , ' | /       \  '  |  :     /     \  
              `----.   \   | /  | |.--.  .-. | |  |   \   /    /  | 
              __ \  \  |   | |  | | \__\/: . . '  : |. \ .    ' / | 
             /  /`--'  /   | |  |/  ,  .--.; | |  | ' \ \'   ;   /| 
            '--'.     /|   | |--'  /  /  ,.  | '  : |--' '   |  / | 
              `--'---' |   |/     ;  :   .'   \;  |,'    |   :    | 
                       '---'      |  ,     .-./'--'       \   \  /  
                                   `--`---'                `----'   "
        };

        Screen::erase_screen();

        // ASCII Art Drawing
        let text_width: u16 = u16::try_from(text.lines()
            .map(|l| l.len())
            .max()
            .unwrap_or(0))
            .unwrap_or(0);
        let start_col = (width.saturating_sub(text_width)) / 2;
        let start_row = 1;
        cursor.jump(start_row, start_col);
        for line in text.lines() {
            print!("{}", line);
            cursor.jump_to_col(start_col);
            cursor.down(1);
        }

        // Options rendering
        let options = &mut self.options;
        let available_options = [MenuOptionType::Play, MenuOptionType::Options, MenuOptionType::Quit];
        for key in available_options.iter() {
            if let Some(option) = options.get_mut(key) {
                cursor.down(2);
                let text_width: u16 = u16::try_from(option.text.len()).unwrap_or(0);
                let start_col = (width.saturating_sub(text_width)) / 2;
                cursor.jump_to_col(start_col);

                if *key == self.selected_option {
                    print!("{ESC}[4m{}", option.text);
                    print!("{ESC}[24m"); // Reset underline
                } else {
                    print!("{}", option.text);
                }

                let position = *cursor.get_position();
                option.set_position(position);
            }
        }

        Screen::flush();
    }
}
