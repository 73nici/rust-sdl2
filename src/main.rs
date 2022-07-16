extern crate sdl2;
extern crate core;

use std::borrow::{Borrow, BorrowMut};
use crate::utils::log_and_exit;

mod utils;
mod game;
mod init;
mod player;

fn main() {
    let sdl2_context = init::sdl2_context();
    let sdl2_video_subsystem = init::sdl2_video_subsystem(sdl2_context.borrow());

    let sdl2_window = init::sdl2_window(sdl2_video_subsystem.borrow(), "Test");
    let mut sdl2_renderer = init::sdl2_renderer(sdl2_window);

    game::game_loop(sdl2_context, sdl2_renderer.borrow_mut());

    println!("Hello, world!");

    log_and_exit(String::from("Finished"), None)
}
