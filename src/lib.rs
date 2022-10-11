use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

static WIDTH: i32 = 640;
static HEIGHT: i32 = 480;

mod color;
mod element;
mod log;
mod perturbable;

use element::{Element, Parent};

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

fn set_initial_transform(context: &mut Context) {
    context.translate(150.0, 150.0);
    context.scale(20.0, 20.0);
}

fn clear(context: &mut Context) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}

fn one_step(root: &mut Element, parent: &mut Parent) {
    root.perturb();
    parent.perturb();

    let context = get_context();
    context.reset_transform();
    clear(&mut context);
    set_initial_transform(&mut context);

    root.draw(&parent, &mut context);
}

fn run(mut root: Element, mut parent: Parent) {
    loop {
        let closure =
            Closure::wrap(Box::new(|| one_step(&mut root, &mut parent)) as Box<dyn FnMut()>);

        get_window().request_animation_frame(closure.as_ref().unchecked_ref());
    }
}

#[wasm_bindgen]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut context = get_context();

    let root = Element::new_random();
    let parent = Parent::new_random();

    run(root, parent);
}
