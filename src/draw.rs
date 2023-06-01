use crate::color::Color;
use crate::{Context, HEIGHT, HEIGHT_IN_PIXELS, UNIT_IN_PIXELS, WIDTH_IN_PIXELS};

use rand::Rng;

mod fool;
mod rock;
mod sky;
mod star;
mod text;

fn set_initial_transform(context: &mut Context) {
    let _ = context.scale(UNIT_IN_PIXELS, -UNIT_IN_PIXELS);
    let _ = context.translate(0.0, -HEIGHT);
}

pub fn draw<R: Rng>(context: &mut Context, mut rng: R) {
    let _ = context.reset_transform();
    context.save();
    set_initial_transform(context);

    sky::draw(context, &mut rng);
    rock::draw(context);
    fool::draw(context, &mut rng);
    text::draw(context);

    context.restore();

    // draw_margins(context)
}

fn draw_margins(context: &mut Context) {
    // let margin_color = "rgba(255, 0, 255, 0.25)";
    let margin_color = "rgb(255, 200, 255)";
    context.set_fill_style(&margin_color.into());

    context.fill_rect(0.0, 0.0, 36.0, HEIGHT_IN_PIXELS);
    context.fill_rect(0.0, 0.0, WIDTH_IN_PIXELS, 36.0);
    context.fill_rect(WIDTH_IN_PIXELS, HEIGHT_IN_PIXELS, -36.0, -HEIGHT_IN_PIXELS);
    context.fill_rect(WIDTH_IN_PIXELS, HEIGHT_IN_PIXELS, -WIDTH_IN_PIXELS, -36.0);

    /*
    let inner_margin_color = "rgba(255, 255, 255, 0.5)";
    context.set_fill_style(&inner_margin_color.into());

    context.fill_rect(36.0, 0.0, 36.0, HEIGHT_IN_PIXELS);
    context.fill_rect(0.0, 36.0, WIDTH_IN_PIXELS, 36.0);
    context.fill_rect(
        WIDTH_IN_PIXELS - 36.0,
        HEIGHT_IN_PIXELS,
        -36.0,
        -HEIGHT_IN_PIXELS,
    );
    context.fill_rect(
        WIDTH_IN_PIXELS,
        HEIGHT_IN_PIXELS - 36.0,
        -WIDTH_IN_PIXELS,
        -36.0,
    );
    */
}
