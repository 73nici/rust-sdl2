use std::sync::Mutex;
use crate::entity::entity::{Entity, EntityTrait};
use crate::entity::player::{Player, PlayerTrait};
use crate::event;
use crate::framework::{Framework};

pub struct Game {
    framework: Mutex<Framework>,
}

impl Game {
    pub(crate) fn new(title: &str) -> Self {
        Game {
            framework: Mutex::new(Framework::new(title)),
        }
    }

    pub fn game_loop(&mut self) {
        let mut player = Player::new(Some(400), Some(300));
        let mut entities = vec![Entity::new(None, None), Entity::new(Some(101), Some(101))];

        'running: loop {
            // let start_tick = self.sdl2_context.timer().unwrap().ticks();
            for event in self.framework.lock().unwrap().get_sdl2_event_pump_mut().poll_iter() {
                if event::handle_event(&event) {
                    break 'running;
                }
            }
        
            self.framework.lock().unwrap().prepare_frame(None);

            entities.retain(|entity| {
                !entity.is_dead
            });

            entities.iter().for_each(|entity| {
                entity.draw(self.framework.lock().unwrap().get_sdl2_renderer_mut());
            });

            player.update(self.framework.lock().unwrap().get_sdl2_event_pump());
            player.check_collision(&mut entities);
            player.draw(self.framework.lock().unwrap().get_sdl2_renderer_mut());

            self.framework.lock().unwrap().show_frame();

            // let end_tick = self.sdl2_context.timer().unwrap().ticks();

            // println!("{}", end_tick - start_tick);
        }
    }
}