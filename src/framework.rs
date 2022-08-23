use sdl2::{Sdl, EventPump};
use sdl2::render::{WindowCanvas};
use sdl2::pixels::{Color};

pub struct Framework {
    sdl2_context: Sdl,
    sdl2_renderer: WindowCanvas,
    sdl2_event_pump: EventPump,
}

impl Framework {
    pub fn new(title: &str) -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(title, 800, 600).opengl().build().unwrap();
        let renderer = window.into_canvas().present_vsync().build().unwrap();
        let event_pump = context.event_pump().unwrap();

        Framework {
            sdl2_context: context,
            sdl2_renderer: renderer,
            sdl2_event_pump: event_pump,
        }
    }

    pub fn prepare_frame(&mut self, color: Option<Color>) {
        self.sdl2_renderer.set_draw_color(color.unwrap_or(Color::RGB(0, 0, 0)));
        self.sdl2_renderer.clear();
    }

    pub fn show_frame(&mut self) {
        self.sdl2_renderer.present();
    }

    #[allow(dead_code)]
    pub fn get_sdl2_context(&self) -> &Sdl {
        &self.sdl2_context
    }

    #[allow(dead_code)]
    pub fn get_sdl2_renderer(&self) -> &WindowCanvas {
        &self.sdl2_renderer
    }

    pub fn get_sdl2_renderer_mut(&mut self) -> &mut WindowCanvas {
        &mut self.sdl2_renderer
    }

    pub fn get_sdl2_event_pump(&self) -> &EventPump {
        &self.sdl2_event_pump
    }

    pub fn get_sdl2_event_pump_mut(&mut self) -> &mut EventPump {
        &mut self.sdl2_event_pump
    }
}

