use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

static WIDTH_IN_PIXELS: f64 = 897.0;
static HEIGHT_IN_PIXELS: f64 = 1497.0;

static HEIGHT: f64 = 42.0;
static UNIT_IN_PIXELS: f64 = HEIGHT_IN_PIXELS / HEIGHT;
static WIDTH: f64 = WIDTH_IN_PIXELS / UNIT_IN_PIXELS;

static RANDOM_SEED: u64 = 27;

static STROKE_WIDTH: f64 = 0.1;

mod color;
mod draw;
mod log;

type Context = web_sys::CanvasRenderingContext2d;

fn get_window() -> Window {
    web_sys::window().unwrap()
}
fn get_context() -> Context {
    let document = get_window().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context
}

fn random() -> impl rand::Rng {
    ChaCha8Rng::seed_from_u64(RANDOM_SEED)
}

fn clear(context: &mut Context) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}

fn run() {
    let mut context = get_context();

    let rng = random();

    // context.set_image_smoothing_enabled(false);

    log::log(format!("HEIGHT: {:}, WIDTH: {:}", HEIGHT, WIDTH).as_str());

    draw::draw(&mut context, rng);
}

#[wasm_bindgen]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    run();
}
