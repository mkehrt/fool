use crate::color::colors;
use crate::{Context, HEIGHT, WIDTH};

pub fn draw(context: &mut Context) {
    context.set_fill_style(&colors::SKY.into());
    context.fill_rect(0.0, 0.0, WIDTH, HEIGHT);
}
