use std::sync::Mutex;
use crate::entity::entity::{Entity, EntityTrait};
use crate::entity::player::{Player};
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

    /// loop of the game
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

            // draw black empty frame
            self.framework.lock().unwrap().prepare_frame(None);

            // draw entities on frame
            entities.iter().for_each(|entity| {
                entity.draw(self.framework.lock().unwrap().get_sdl2_renderer_mut());
            });

            // update player position by movement
            player.update(self.framework.lock().unwrap().get_sdl2_event_pump());

            // check if entities are still alive
            entities.retain(|entity| {
                !player.entity.check_collision(entity)
            });

            // draw player to frame
            player.entity.draw(self.framework.lock().unwrap().get_sdl2_renderer_mut());

            self.framework.lock().unwrap().show_frame();
        }
    }
}