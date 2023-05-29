use std::f64::consts::PI;

use crate::color::Color;
use crate::Context;

use crate::log::log;

// From https://people.sc.fsu.edu/~jburkardt/datasets/polygon/polygon.html
fn star_points() -> Vec<(f64, f64)> {
    vec![
        (0.95105651629515353, 0.30901699437494740),
        (0.22451398828979272, 0.30901699437494740),
        (-0.95105651629515353, 0.30901699437494751),
        (-0.36327126400268051, -0.11803398874989464),
        (0.58778525229247292, -0.80901699437494756),
        (0.36327126400268039, -0.11803398874989492),
        (0.0, 1.0000000000000000),
        (-0.22451398828979263, 0.30901699437494745),
        (-0.58778525229247325, -0.80901699437494734),
        (0.0, -0.38196601125010515),
    ]
}

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

    let star_points = star_points();

    let (start_x, start_y) = star_points.last().unwrap();
    context.move_to(*start_x, *start_y);

    for (x, y) in star_points {
        context.line_to(x, y);
    }

    context.set_fill_style(&color.into());
    context.fill();

    // log(format!("Star: {}, {}, {}", x, y, radius).as_str());

    context.restore();
}
