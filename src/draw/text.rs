use crate::color::colors;
use crate::{Context, HEIGHT, WIDTH};

pub fn draw(context: &mut Context) {
    context.set_font("2.3px gotham-medium");
    context.set_text_align("center");

    context.set_fill_style(&colors::FOOL.into());
    context
        .fill_text("0. The Fool", WIDTH / 2.0, 5.3)
        .expect("Fill text");

    context.set_font("0.2px gotham-book");
    context.set_text_align("right");
    context.set_text_baseline("bottom");

    context.set_fill_style(&colors::ROCK_DARK.into());
    context
        .fill_text("MATTHEW W. KEHRT", WIDTH - 1.1, HEIGHT - 1.1)
        .expect("Signature");
}
