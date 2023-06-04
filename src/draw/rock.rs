use crate::color::{colors, Color};
use crate::{Context, HEIGHT, STROKE_WIDTH};

pub fn draw(context: &mut Context) {
    // Draw from the bottom.
    context.save();
    context.scale(1.0, -1.0).expect("Scale rocks");
    context.translate(0.0, -HEIGHT).expect("Tranlate rocks");

    draw_one(context, draw_top, colors::ROCK_LIGHT);
    draw_one(context, draw_middle, colors::ROCK_MEDIUM);
    draw_one(context, draw_bottom, colors::ROCK_DARK);

    context.restore();
}

pub fn draw_one<F>(context: &mut Context, draw: F, color: Color)
where
    F: FnOnce(&mut Context) -> (),
{
    context.begin_path();

    draw(context);

    context.set_fill_style(&color.into());
    context.fill();

    context.set_line_width(STROKE_WIDTH);
    context.set_stroke_style(&colors::ROCK_STROKE.into());
    context.stroke();
}

pub fn draw_top(context: &mut Context) {
    context.line_to(0.0, 5.644057125);

    context.line_to(10.502166838, 9.382228583);
    context.line_to(10.0, 8.0);
    context.line_to(0.0, 3.56043956);
}

pub fn draw_middle(context: &mut Context) {
    context.move_to(0.0, 3.56043956);

    context.line_to(8.5, 7.296703297);
    context.line_to(5.0, 4.0);
    context.line_to(0.0, 0.8902439023);
}

pub fn draw_bottom(context: &mut Context) {
    context.move_to(0.0, 0.8902439023);

    context.line_to(4.0, 3.329268293);
    context.line_to(3.0, 0.0);
    context.line_to(1.0, -1.0);
    context.line_to(-1.0, -1.0);
}
