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

pub fn init() -> Player {
    let player: Player = Player {
        position: Position {
            x: 0,
            y: 0,
        },
        shape: Rect::new(0, 0, 100, 100),
        color: Color::RGB(255, 255, 255),
    };

    return player;
}

pub fn update(mut player: &mut Player, event_pump: &EventPump) {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Down) {
        player.position.y += 1;
        player.shape.y += 1;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Up) {
        player.position.y -= 1;
        player.shape.y -= 1;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
        player.position.x += 1;
        player.shape.x += 1;
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
        player.position.x -= 1;
        player.shape.x -= 1;
    }
}

pub fn draw(player: &Player, renderer: &mut WindowCanvas) {
    renderer.set_draw_color(player.color);
    renderer.draw_rect(player.shape).unwrap();
}