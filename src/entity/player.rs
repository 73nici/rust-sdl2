use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use crate::entity::entity::{Entity, EntityTrait};

pub struct Player {
    entity: Entity,
}

pub trait PlayerTrait {
    fn update(&mut self, event_pump: &EventPump);
    fn draw(&self, renderer: &mut WindowCanvas);
    fn check_collision(&self, entitiy: &Entity) -> bool;
    fn new(x: Option<i32>, y: Option<i32>) -> Self;
}


impl PlayerTrait for Player {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Player {
            entity: Entity::new(x, y)
        }
    }

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

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(self.entity.color);
        renderer.draw_rect(self.entity.shape).unwrap();
    }

    fn check_collision(&self, entity: &Entity) -> bool {

        if (self.entity.shape.x() <= entity.shape.x() + i32::try_from(entity.shape.width()).unwrap())
            && (self.entity.shape.x() + i32::try_from(self.entity.shape.width()).unwrap() > entity.shape.x())
            && (self.entity.shape.y() <= entity.shape.y() + i32::try_from(entity.shape.width()).unwrap())
            && (self.entity.shape.y() + i32::try_from(self.entity.shape.width()).unwrap() > entity.shape.y)
            {
                return true;
            }
        return false;
    }
}