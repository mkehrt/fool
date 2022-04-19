use pathfinder_canvas::{CanvasFontContext, CanvasRenderingContext2D, Path2D, Canvas};
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::fs::FilesystemResourceLoader;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

static WIDTH: i32 = 640;
static HEIGHT: i32 = 480;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Make sure we have at least a GL 3.0 context. Pathfinder requires this.
    let gl_attributes = video.gl_attr();
    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_version(3, 3);

    // Open a window.
    let window_size = Vector2I::new(WIDTH, HEIGHT);
    let window = video.window("Fool", window_size.x() as u32, window_size.y() as u32)
                      .opengl()
                      .build()
                      .unwrap();

    // Create the GL context, and make it current.
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
    window.gl_make_current(&gl_context).unwrap();

    let mut renderer = Renderer::new(GLDevice::new(GLVersion::GL3, 0),
    &FilesystemResourceLoader::locate(),
    DestFramebuffer::full_window(window_size),
    RendererOptions { background_color: Some(ColorF::white()) });

    // Make a canvas. We're going to draw a house.
    let canvas =  Canvas::new(Vector2F::new(WIDTH as f32, HEIGHT as f32));
    let context: CanvasRenderingContext2D = canvas.get_context_2d(CanvasFontContext::from_system_source());

    // Render the canvas to screen.
    let canvas = context.into_canvas();
    let scene = SceneProxy::from_scene(canvas.into_scene(), RayonExecutor);
    scene.build_and_render(&mut renderer, BuildOptions::default());
    window.gl_swap_window();

    // Wait for a keypress.
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        match event_pump.wait_event() {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            _ => {}
        }
    }
}
