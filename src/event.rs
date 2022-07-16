use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn handle_keyboard_event(event: &Event, mut is_close: bool) -> bool {
    match event {
        Event::KeyDown {scancode, ..} => {
            if scancode.unwrap() == Scancode::Escape {
                is_close = true;
            }
        },
        _ => (),
    }
    return is_close;
}

fn handle_close_event(event: &Event, mut is_close: bool) -> bool {
    match event {
        Event::Quit {..} => {
            is_close = true;
        },
        _ => (),
    }
    return is_close;
}

pub fn handle_event(event: &Event) -> bool {
    let mut is_close = false;
    is_close = handle_keyboard_event(event, is_close);
    is_close = handle_close_event(event, is_close);

    return is_close;
}