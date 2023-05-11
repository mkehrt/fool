use crate::Context;

use rand::Rng;

mod fool;
mod rock;
mod sky;
mod star;

pub fn draw<R: Rng>(context: &mut Context, rng: &mut R) {
    sky::draw(context, rng);
    // rock::draw(context);
    fool::draw(context);
}
