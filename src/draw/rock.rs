use crate::color::colors;
use crate::{Context, WIDTH};

static TOE_Y: f64 = 8.558967936;
static TOE_X: f64 = 8.05;

pub fn draw(context: &mut Context) {
    context.move_to(0.0, 0.0);

    context.line_to(0.0, 5.0);
    context.line_to(TOE_Y, TOE_X);
    context.line_to(5.0, 3.0);
    context.line_to(2.0, 0.0);
    context.line_to(0.0, 0.0);

    context.set_fill_style(&colors::ROCK.into());
    context.fill();
}
