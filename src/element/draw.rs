use crate::color::Color;
use crate::element::{Element, Parent};
use crate::perturbable::Perturbable;
use crate::Context;

use num::Float;

pub fn draw(
    context: &mut Context,
    depth: u64,
    length_scale: f64,
    element: &Element,
    parent: &Parent,
) {
    if depth > 1 {
        return;
    }
    let angle = element.angle() / std::f64::consts::PI * 180.0;
    if angle > 90.0 {
        crate::console_log!("Angle: {:}", angle);
    }

    context.save();

    let height = element.height();
    let angle = element.angle();

    context.scale(height, height).unwrap();

    if angle < 0.0 {
        context.begin_path();
        let radius = -2.0 * std::f64::consts::PI / angle;
        let center_x = -radius;
        let center_y = 0.0;

        context
            .ellipse_with_anticlockwise(center_x, center_y, radius, radius, 0.0, 0.0, angle, true)
            .unwrap();

        context.set_line_width(1.0);
        context.stroke();
    } else if angle == 0.0 {
    } else
    /* angle > 0.0 */
    {
        context.begin_path();
        let radius = 2.0 * std::f64::consts::PI / angle;
        let center_x = radius;
        let center_y = 0.0;

        context
            .ellipse(
                center_x,
                center_y,
                radius,
                radius,
                0.0,
                std::f64::consts::PI,
                -angle,
            )
            .unwrap();

        context.set_line_width(1.0);
        context.stroke();
    }

    /*
    let intermediate_x = Float::sin(element.angle() / 2.0) * height / 2.0;
    let intermediate_y = -Float::cos(element.angle() / 2.0) * height / 2.0;

    let distal_x = Float::sin(element.angle()) * height;
    let distal_y = -Float::cos(element.angle()) * height;
    */

    //context.move_to(-0.5, 0.0);
    //context.line_to(0.5, 0.0);
    //    context.move_to(0.0, 0.0);
    //   context
    //       .arc_to(intermediate_x, intermediate_y, distal_x, distal_y, height)
    //       .unwrap();
    // context.line_to, -1.0);
    // context.line_to(-0.5, -1.0);
    //context.line_to(-0.5, 0.0);

    //    set_gradient(context, parent.color, element.distal_color);
    //context.fill();
    //  context.set_line_width(1.0 / length_scale);
    //  context.stroke();

    context.restore();

    for (index, child) in element.children.children.iter().enumerate() {
        draw_child(context, depth, length_scale, element, &child, index);
    }
}

fn draw_child(
    context: &mut Context,
    depth: u64,
    length_scale: f64,
    element: &Element,
    child: &Element,
    index: usize,
) {
    context.save();

    context.rotate(element.angle()).unwrap();
    context
        .translate(0.0, -element.height() * length_scale)
        .unwrap();

    let width = 1.0 / element.children.children.len() as f64;
    let translate = (-0.5 + (width * index as f64)) + (width / 2.0);

    context.translate(translate, 0.0).unwrap();
    context.scale(width, width).unwrap();

    let parent = Parent {
        color: element.distal_color,
    };

    draw(
        context,
        depth + 1,
        length_scale / element.height(),
        child,
        &parent,
    );

    context.restore()
}

fn set_gradient(context: &mut Context, _proximal: Color, distal: Color) {
    let style = format!(
        "rgb({:}, {:}, {:})",
        distal.red.get_value(),
        distal.green.get_value(),
        distal.blue.get_value()
    );
    context.set_stroke_style(&style.into());
}
