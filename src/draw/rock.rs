use crate::color::colors;
use crate::{Context, WIDTH};

pub fn draw(context: &mut Context) {
    context.move_to(0.0, 0.0);

    context.line_to(0.0, 5.0);
    context.line_to(WIDTH / 3.0, 8.0);
    context.line_to(5.0, 3.0);
    context.line_to(2.0, 0.0);
    context.line_to(0.0, 0.0);

    context.set_fill_style(&colors::ROCK.into());
    context.fill();
}
