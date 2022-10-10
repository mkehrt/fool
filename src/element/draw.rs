use crate::color::Color;
use crate::Context;
use crate::element::Element;
use crate::perturbable::Perturbable;

use super::Parent;

pub fn draw(context: &mut Context, element: &Element, parent: &Parent) {
    context.begin_path();

    context.move_to(-0.5, 0.0);
    context.line_to(0.5, 0.0);
    context.line_to(0.5, 1.0);
    context.line_to(-0.5, 1.0);
    context.line_to(-0.5, 0.0);

    set_gradient(context, parent.color, element.distal_color);
    context.fill();
}

fn set_gradient(context: &mut Context, _proximal: Color, distal: Color) {
    let style = format!("rgb({:}, {:}, {:})", distal.red.get_value(), distal.green.get_value(), distal.blue.get_value());
    context.set_fill_style(&style.into());
}
