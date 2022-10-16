use sdl2::event::Event;
use sdl2::keyboard::Scancode;

/// handles keyboard close events
fn handle_keyboard_event(event: &Event) -> bool {
    match event {
        Event::KeyDown {scancode, ..} => {
            if scancode.unwrap() == Scancode::Escape || scancode.unwrap() == Scancode::Q {
                return true
            }
            false
        },
        _ => false,
    }
}

/// handles window close events
fn handle_close_event(event: &Event) -> bool {
    match event {
        Event::Quit {..} => {
            true
        },
        _ => false
    }
}

/// handles all (currently only close) events
pub fn handle_event(event: &Event) -> bool {
    let is_close = handle_keyboard_event(event);

    if is_close {
        return is_close
    }

    let is_close = handle_close_event(event);
    return is_close;
}