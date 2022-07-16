use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Player {
    shape: Rect,
    position: Position,
    color: Color,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            position: Position {
                x: 0,
                y: 0,
            },
            shape: Rect::new(0, 0, 100, 100),
            color: Color::RGB(255, 255, 255),
        }
    }

    pub(crate) fn update(&mut self, event_pump: &EventPump) {
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Down) {
            self.position.y += 1;
            self.shape.y += 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Up) {
            self.position.y -= 1;
            self.shape.y -= 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
            self.position.x += 1;
            self.shape.x += 1;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
            self.position.x -= 1;
            self.shape.x -= 1;
        }
    }

    pub(crate) fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(self.color);
        renderer.draw_rect(self.shape).unwrap();
    }
}
