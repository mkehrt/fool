use crate::{Context, UNIT_IN_PIXELS};

use rand::Rng;

mod fool;
mod margins;
mod rock;
mod sky;
mod star;
mod stars;
mod text;

fn set_initial_transform(context: &mut Context) {
    let _ = context.scale(UNIT_IN_PIXELS, UNIT_IN_PIXELS);
}

pub fn draw<R: Rng>(context: &mut Context, mut rng: R) {
    let _ = context.reset_transform();
    context.save();
    set_initial_transform(context);

    sky::draw(context);
    rock::draw(context);
    fool::draw(context, &mut rng);
    text::draw(context);
    stars::draw(context, &mut rng);

    context.restore();

    // margins::draw(context, false);
}
