use rand::Rng;

use crate::color::colors;
use crate::{Context, HEIGHT, WIDTH};

use super::star;

pub fn draw(context: &mut Context) {
    context.set_fill_style(&colors::SKY.into());
    context.fill_rect(0.0, 0.0, WIDTH, HEIGHT);
}
