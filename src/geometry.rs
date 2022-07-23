use wasm_bindgen::prelude::*;

pub fn draw(context: &mut web_sys::CanvasRenderingContext2d) {
     println!("AAAAAAA");

    context.begin_path();

    context.set_fill_style(&"blue".into());
    context.move_to(0.0,0.0);
    context.line_to(10.0,0.0);
    context.line_to(0.0,10.0);
    context.line_to(0.0,0.0);
    context.fill();
}
