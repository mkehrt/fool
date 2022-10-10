use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

static WIDTH: i32 = 640;
static HEIGHT: i32 = 480;

mod children;
mod color;
mod element;
mod log;
mod perturbable;


use color::{Blue, Color, Green, Red};
use children::Children;
use element::{Element, Parent};
use perturbable::Perturbable;

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
    context.scale(20.0, 20.0);
}

fn clear(context: &mut Context) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}

#[wasm_bindgen]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut context = get_context();

    let mut root = Element {
        distal_color: Color {
            red: Red::new_with_value(255),
            green: Green::new_with_value(0),
            blue: Blue::new_with_value(255),
        },
        height: element::Height::new_with_value(1.0),
        angle: element::Angle::new_with_value(0.0),
        children: Children::new(),
    };
    let root_parent = Parent {
        color: Color {
            red: Red::new_with_value(255),
            green: Green::new_with_value(0),
            blue: Blue::new_with_value(255),
        },
    };

    set_initial_transform(&mut context);

    loop {
        console_log!("{:?}", root);
        root.draw(&root_parent, &mut context);
        root = root.perturb();
        clear(&mut context);
    }
}

