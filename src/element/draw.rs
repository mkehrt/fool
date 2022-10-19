use crate::color::Color;
use crate::element::{Element, Parent};
use crate::perturbable::Perturbable;
use crate::Context;

pub fn draw(context: &mut Context, depth: u64, element: &Element, parent: &Parent) {
    context.save();

    let height = element.height();
    context.scale(1.0, height).unwrap();

    context.begin_path();

    if depth == 4 {
        context.move_to(-0.5, 0.0);
        context.line_to(0.5, 0.0);
        context.line_to(0.5, -1.0);
        context.line_to(-0.5, -1.0);
        context.line_to(-0.5, 0.0);

        set_gradient(context, parent.color, element.distal_color);
        context.fill();
    }

    context.restore();

    for (index, child) in element.children.children.iter().enumerate() {
        draw_child(context, depth, element, &child, index);
        break;
    }
}

fn draw_child(context: &mut Context, depth: u64, element: &Element, child: &Element, index: usize) {
    context.save();

    let rotation = element.angle.value * std::f64::consts::PI / 180.0;
    context.rotate(rotation).unwrap();

    let height = element.height();
    context.translate(0.0, -height).unwrap();

    let width = 1.0 / element.children.children.len() as f64;
    let translate = (-0.5 + (width * index as f64)) + (width / 2.0);

    context.translate(translate, 0.0).unwrap();
    context.scale(width, 1.0).unwrap();

    let parent = Parent {
        color: element.distal_color,
    };

    draw(context, depth + 1, child, &parent);

    context.restore()
}

fn set_gradient(context: &mut Context, _proximal: Color, distal: Color) {
    let style = format!(
        "rgb({:}, {:}, {:})",
        distal.red.get_value(),
        distal.green.get_value(),
        distal.blue.get_value()
    );
    context.set_fill_style(&style.into());
}
