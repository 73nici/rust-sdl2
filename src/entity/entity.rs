use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::entity::position::Position;

pub struct Entity {
    pub position: Position,
    pub shape: Rect,
    pub color: Color,
    pub is_dead: bool
}

pub trait EntityTrait {
    fn new(x: Option<i32>, y: Option<i32>) -> Self;
    fn draw(&self, renderer: &mut WindowCanvas);
}

impl EntityTrait for Entity {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        let x = x.unwrap_or(0);
        let y = y.unwrap_or(0);


        Entity {
            position: Position::new(None, None),
            shape: Rect::new(x, y, 100, 100),
            color: Color::RGB(255, 255, 255),
            is_dead: false,
        }
    }
    
    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.draw_rect(self.shape).unwrap();
    }
}