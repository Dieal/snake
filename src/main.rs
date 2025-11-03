use std::fs::File;
use simplelog::{Config, LevelFilter, WriteLogger};
use snake::{game::SnakeGame, menu::Menu};

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("snake.log").unwrap(),
    );

    // let mut game: SnakeGame = SnakeGame::new();
    // game.run();
    let mut main_menu: Menu = Menu::new();
    main_menu.run();
}
