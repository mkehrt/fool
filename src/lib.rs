use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

static WIDTH: i32 = 640;
static HEIGHT: i32 = 480;

mod geometry;

fn get_context() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
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

#[wasm_bindgen]
pub fn start() {
    println!("AAAAAAA");
    let mut context = get_context();
    geometry::draw(&mut context);
}
