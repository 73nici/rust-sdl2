use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use crate::entity::entity::{EntityTrait, Entity};

pub struct Player {
    entity: Entity,
}

pub trait PlayerTrait {
    fn update(&mut self, event_pump: &EventPump);
    fn check_collision(&mut self, entities: &mut Vec<Entity>);
}

impl EntityTrait for Player {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Player {
            entity: Entity::new(x, y),
        }
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(self.entity.color);
        renderer.draw_rect(self.entity.shape).unwrap();
    }
}


impl PlayerTrait for Player {
    fn update(&mut self, event_pump: &EventPump) {
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Down) {
            self.entity.position.y += 1;
            self.entity.shape.y += 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Up) {
            self.entity.position.y -= 1;
            self.entity.shape.y -= 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
            self.entity.position.x += 1;
            self.entity.shape.x += 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
            self.entity.position.x -= 1;
            self.entity.shape.x -= 1;
        }
    }

    fn check_collision(&mut self, entities: &mut Vec<Entity>) {
        entities.iter_mut().for_each(|mut entity| {
            if entity.position.check_collision(&self.entity.position) {
                println!("collision");
                entity.is_dead = true;
            }
        });
    }
}