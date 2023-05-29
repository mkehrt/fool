use std::f64::consts::PI;

use crate::color::Color;
use crate::Context;

use crate::log::log;

pub fn draw(
    context: &mut Context,
    color: Color,
    x: f64,
    y: f64,
    radius: f64,
    angle_in_degrees: f64,
) {
    context.save();

    let _ = context.translate(x, y);
    let _ = context.scale(radius, radius);

    let angle_in_radians = angle_in_degrees / 360.0 * 2.0 * PI;
    let _ = context.rotate(angle_in_radians);

    // TODO draw a star, centered at 0, 0, radius 0.5.
    context.set_fill_style(&color.into());
    context.fill_rect(-0.5, -0.5, 1.0, 1.0);

    // log(format!("Star: {}, {}, {}", x, y, radius).as_str());

    context.restore();
}
