use crate::Context;

use rand::Rng;

mod rock;
mod sky;
mod star;

pub fn draw<R: Rng>(context: &mut Context, rng: &mut R) {
    sky::draw(context, rng);
    rock::draw(context);
}
