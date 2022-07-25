use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

static WIDTH: i32 = 640;
static HEIGHT: i32 = 480;

mod color;
mod element;

use color::Color;
use element::{Element, Parent};

type Context = web_sys::CanvasRenderingContext2d;

fn get_context() -> Context {
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

fn set_initial_transform(context: &mut Context) {
    context.translate(150.0, 150.0);
    context.scale(-20.0, -20.0);
}

#[wasm_bindgen]
pub fn start() {
    let mut context = get_context();

    let root = Element {
        distal_color: Color {
            red: 255,
            green: 0,
            blue: 255,
        },
        height: 1.0,
        angle: 0.0,
        children: Vec::new(),
    };
    let root_parent = Parent {
        color: Color {
            red: 255,
            green: 0,
            blue: 255,
        },
    };

    set_initial_transform(&mut context);
    root.draw(&root_parent, &mut context);
}
