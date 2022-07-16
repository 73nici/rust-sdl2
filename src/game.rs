use std::borrow::{Borrow, BorrowMut};
use sdl2::{Sdl};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::{log_and_exit, player};


pub fn game_loop(sdl_context: Sdl, renderer: &mut WindowCanvas) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut my_player = player::init();

    'running: loop {
        let start_tick = sdl_context.timer().unwrap().ticks();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    log_and_exit(String::from("Close"), None);
                    break 'running;
                },
                Event::KeyDown {keycode, ..} => {
                    println!("KeyDown, {}", keycode.unwrap());
                },
                _ => (),
            }
        }



        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        player::update(my_player.borrow_mut(), event_pump.borrow());
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        player::draw(my_player.borrow(), renderer.borrow_mut());
        renderer.present();

        let end_tick = sdl_context.timer().unwrap().ticks();

        println!("{}", end_tick - start_tick);
    }
}