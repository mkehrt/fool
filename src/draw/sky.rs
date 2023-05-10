use rand::Rng;

use crate::color::colors;
use crate::{Context, HEIGHT, WIDTH};

use super::star;

const NUM_STARS: u64 = 75;
const MAX_STAR_RADIUS: f64 = 0.35;

pub fn draw<R: Rng>(context: &mut Context, mut rng: R) {
    context.set_fill_style(&colors::SKY.into());
    context.fill_rect(0.0, 0.0, WIDTH, HEIGHT);

    for _ in 0..NUM_STARS {
        let star_x = rng.gen_range(0.0..WIDTH - MAX_STAR_RADIUS * 2.0) + MAX_STAR_RADIUS;
        let star_y = rng.gen_range(0.0..HEIGHT - MAX_STAR_RADIUS * 2.0) + MAX_STAR_RADIUS;
        let star_radius = rng.gen_range(0.0..MAX_STAR_RADIUS);
        let star_angle = rng.gen_range(0.0..360.0);

        star::draw(
            context,
            colors::SKY_STARS,
            star_x,
            star_y,
            star_radius,
            star_angle,
        );
    }
}
