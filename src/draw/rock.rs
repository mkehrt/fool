use crate::color::{colors, Color};
use crate::{Context, STROKE_WIDTH, WIDTH};

static TOE_X: f64 = 8.558967936;
static TOE_Y: f64 = 9.05;

static EPSILON: f64 = 0.0;

pub fn draw(context: &mut Context) {
    draw_one(context, draw_top, colors::ROCK_LIGHT);
    draw_one(context, draw_middle, colors::ROCK_MEDIUM);
    draw_one(context, draw_bottom, colors::ROCK_DARK);
}

pub fn draw_one<F>(context: &mut Context, draw: F, color: Color)
where
    F: FnOnce(&mut Context) -> (),
{
    context.begin_path();

    draw(context);

    // context.close_path();

    context.set_fill_style(&color.into());
    context.fill();

    context.set_line_width(STROKE_WIDTH);
    context.set_stroke_style(&colors::ROCK_STROKE.into());
    context.stroke();
}

pub fn draw_top(context: &mut Context) {
    context.line_to(-EPSILON, 6.0);

    context.line_to(9.502166838, 9.382228583);
    context.line_to(9.0, 8.0);
    context.line_to(-EPSILON, 4.0);
}

pub fn draw_middle(context: &mut Context) {
    context.move_to(-EPSILON, 4.0);

    context.line_to(7.5, 7.296703297);
    context.line_to(4.0, 4.0);
    context.line_to(-EPSILON, 1.5);
}

pub fn draw_bottom(context: &mut Context) {
    context.move_to(-EPSILON, 1.5);

    context.line_to(3.0, 3.329268293);
    context.line_to(2.0, -EPSILON);
    context.line_to(-EPSILON, -1.0);
}
