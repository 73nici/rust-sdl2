use crate::entity::entity::{Entity, EntityTrait};
use sdl2::keyboard::Scancode;
use sdl2::EventPump;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Player {
            entity: Entity::new(x, y),
        }
    }

    pub fn update(&mut self, event_pump: &EventPump) {
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::Down)
        {
            self.entity.position.y += 1;
            self.entity.shape.y += 1;
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::Up)
        {
            self.entity.position.y -= 1;
            self.entity.shape.y -= 1;
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::Right)
        {
            self.entity.position.x += 1;
            self.entity.shape.x += 1;
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::Left)
        {
            self.entity.position.x -= 1;
            self.entity.shape.x -= 1;
        }
    }
}
