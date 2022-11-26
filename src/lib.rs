use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

static WIDTH: i32 = 800;
static HEIGHT: i32 = 800;

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
    let _ = context.translate(400.0, 750.0);
    let _ = context.scale(50.0, 50.0);
}

fn clear(context: &mut Context) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}

fn one_step(root: &mut Element, parent: &mut Parent) -> (Element, Parent) {
    let root = root.perturb();
    let parent = parent.perturb();

    let mut context = get_context();
    let _ = context.reset_transform();
    clear(&mut context);
    set_initial_transform(&mut context);

    root.draw(&parent, &mut context);

    /*
    let (height_count, total_height) = root.height_statistics();
    let average_height = total_height / height_count as f64;
    console_log!("Average height: {:}", average_height);
    */

    (root, parent)
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let _ = get_window().request_animation_frame(f.as_ref().unchecked_ref());
}

fn run(mut root: Element, mut parent: Parent) {
    // Deep magic from before the dawn of time^W^W^W^W^W https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let internal_closure = Rc::new(RefCell::new(None));
    let external_closure = internal_closure.clone();

    *external_closure.borrow_mut() = Some(Closure::new(move || {
        let (new_root, new_parent) = one_step(&mut root, &mut parent);
        root = new_root;
        parent = new_parent;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(internal_closure.borrow().as_ref().unwrap());
    }));

    request_animation_frame(external_closure.borrow().as_ref().unwrap());
}

#[wasm_bindgen]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // TODO background color
    // TODO root width
    let root = Element::new_random();
    let parent = Parent::new_random();

    run(root, parent);
}
