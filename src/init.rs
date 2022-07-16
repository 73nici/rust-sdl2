use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::Sdl;

pub fn sdl2_context() -> Sdl {
    let sdl_context = sdl2::init().unwrap();
    return sdl_context;
}

pub fn sdl2_video_subsystem(sdl2_context: &Sdl) -> VideoSubsystem {
    let video_subsystem = sdl2_context.video().unwrap();
    return video_subsystem;
}

pub fn sdl2_window(video_subsystem: &VideoSubsystem, title: &str) -> Window  {
    let sdl_window = video_subsystem.window(title, 800, 600).opengl().build().unwrap();
    return sdl_window;
}

pub fn sdl2_renderer(window: Window) -> WindowCanvas {
    let mut renderer = window.into_canvas().present_vsync().build().unwrap();
    renderer.set_draw_color(Color::RGB(200, 200, 200));
    renderer.clear();
    renderer.present();
    return renderer;
}

