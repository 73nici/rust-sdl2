use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::entity::position::Position;

pub struct Entity {
    pub position: Position,
    pub shape: Rect,
    pub color: Color,
}

pub trait EntityTrait {
    fn new(x: Option<i32>, y: Option<i32>) -> Self;
    fn draw(&self, renderer: &mut WindowCanvas);
    fn check_collision(&self, entities: &Entity) -> bool;
}

impl EntityTrait for Entity {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        let x = x.unwrap_or(0);
        let y = y.unwrap_or(0);

        Entity {
            position: Position::new(Some(x), Some(y)),
            shape: Rect::new(x, y, 100, 100),
            color: Color::RGB(255, 255, 255),
        }
    }
    
    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.draw_rect(self.shape).unwrap();
    }

    fn check_collision(&self, entity: &Entity) -> bool {

        if (self.shape.x() <= entity.shape.x() + i32::try_from(entity.shape.width()).unwrap())
            && (self.shape.x() + i32::try_from(self.shape.width()).unwrap() > entity.shape.x())
            && (self.shape.y() <= entity.shape.y() + i32::try_from(entity.shape.width()).unwrap())
            && (self.shape.y() + i32::try_from(self.shape.width()).unwrap() > entity.shape.y)
            {
                return true;
            }
        return false;
    }
}