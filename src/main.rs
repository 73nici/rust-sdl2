extern crate sdl2;
extern crate core;

use crate::game::Game;

mod utils;
mod game;
mod event;
mod entity;
mod framework;

/// entry point of game
fn main() {
    // create game struct and start game loop
    let mut game = Game::new("Test");

    game.game_loop();
    utils::log_and_exit("Finished", None);
}
