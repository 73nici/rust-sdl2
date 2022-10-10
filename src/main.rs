extern crate sdl2;
extern crate core;

use crate::utils::log_and_exit;
use crate::game::Game;

mod utils;
mod game;
mod event;
mod entity;
mod framework;

fn main() {
    let mut game = Game::new("Test");

    game.game_loop();
    log_and_exit("Finished", None);
}
