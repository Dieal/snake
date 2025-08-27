use std::fs::File;

use ascii::SnakeGame;
use simplelog::{Config, LevelFilter, WriteLogger};

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("my_rust_binary.log").unwrap(),
    );

    let mut game: SnakeGame = SnakeGame::new();
    game.run();
}
