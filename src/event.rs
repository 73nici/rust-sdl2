use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn handle_keyboard_event(event: &Event, is_close: &mut bool) {
    match event {
        Event::KeyDown {scancode, ..} => {
            if scancode.unwrap() == Scancode::Escape || scancode.unwrap() == Scancode::Q
            {
                *is_close = true;
            }
        },
        _ => (),
    }
}

fn handle_close_event(event: &Event, is_close: &mut bool) {
    match event {
        Event::Quit {..} => {
            *is_close = true;
        },
        _ => (),
    }
}

pub fn handle_event(event: &Event) -> bool {
    let mut is_close = false;
    handle_keyboard_event(event, &mut is_close);
    handle_close_event(event, &mut is_close);

    return is_close;
}