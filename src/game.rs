use sdl2::{Sdl};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::event;
use crate::player::Player;

pub struct Game {
    sdl2_context: Sdl,
    sdl2_renderer: WindowCanvas,
}

impl Game {
    pub(crate) fn new(sdl2_context: Sdl, sdl2_renderer: WindowCanvas) -> Self {
        Game {
            sdl2_context,
            sdl2_renderer
        }
    }

    pub fn game_loop(&mut self) {
        let mut event_pump = self.sdl2_context.event_pump().unwrap();
        let mut player = Player::new();

        'running: loop {
            let start_tick = self.sdl2_context.timer().unwrap().ticks();
            for event in event_pump.poll_iter() {
                if event::handle_event(&event) {
                    break 'running;
                }
            }

            self.sdl2_renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.sdl2_renderer.clear();

            player.update(&event_pump);
            player.draw(&mut self.sdl2_renderer);
            self.sdl2_renderer.present();

            let end_tick = self.sdl2_context.timer().unwrap().ticks();

            println!("{}", end_tick - start_tick);
        }
    }
}