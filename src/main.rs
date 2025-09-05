use std::fs::File;

use simplelog::{Config, LevelFilter, WriteLogger};
use snake::SnakeGame;

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("my_rust_binary.log").unwrap(),
    );

    let mut game: SnakeGame = SnakeGame::new();
    game.run();
}
