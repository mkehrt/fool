use glfw::{Context as _, WindowEvent};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::sync::mpsc;

static WIDTH: u32 = 897;
static HEIGHT: u32 = 1497;

static NAME: &str = "The Fool";

fn make_surface() -> GlfwSurface {
    let dim = WindowDim::Windowed {
        width: WIDTH,
        height: HEIGHT,
    };
    let opt = WindowOpt::default().set_dim(dim);
    let surface = GlfwSurface::new_gl33(NAME, opt);
    surface.unwrap()
}

fn run(mut surface: GlfwSurface) {
    'app: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            if !handle_event(event) {
                break 'app;
            };
        }
        surface.context.window.swap_buffers();
    }
}

fn handle_event(event: WindowEvent) -> bool {
    match event {
        WindowEvent::Close => false,
        _ => true,
    }
}

fn main() {
    let surface = make_surface();

    run(surface)
}
