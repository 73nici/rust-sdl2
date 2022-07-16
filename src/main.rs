extern crate sdl2;
extern crate core;

use crate::utils::log_and_exit;
use crate::game::Game;

mod utils;
mod game;
mod init;
mod player;
mod event;
mod sprite;

fn main() {
    let sdl2_context = init::sdl2_context();
    let sdl2_video_subsystem = init::sdl2_video_subsystem(&sdl2_context);

    let sdl2_window = init::sdl2_window(&sdl2_video_subsystem, "Test");
    let sdl2_renderer = init::sdl2_renderer(sdl2_window);

    let mut game = Game::new(sdl2_context, sdl2_renderer);
    game.game_loop();
    log_and_exit(String::from("Finished"), None)
}
