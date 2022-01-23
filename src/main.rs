use glfw::{Context as _, WindowEvent};
use luminance_glfw::GlfwSurface;
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
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

fn run(mut surface: GlfwSurface) -> Result<(), ()> {
    loop {
        handle_event(&mut surface)?;
        render(&mut surface)?;
        surface.context.window.swap_buffers();
    }
}

fn handle_event(surface: &mut GlfwSurface) -> Result<(), ()> {
    surface.context.window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&surface.events_rx) {
        match event {
            WindowEvent::Close => Err(())?,
            _ => (),
        }
    }

    Ok(())
}

fn render(surface: &mut GlfwSurface) -> Result<(), ()> {
    let color = [1.0, 1.0, 0.0, 1.0];

    let back_buffer = surface.context.back_buffer().unwrap();

    surface.context
    .new_pipeline_gate()
    .pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(color),
      |_, _| Ok(()),
    )
    .assume();
    Ok(())
}

fn main() {
    let surface = make_surface();
    let _ = run(surface);
}
