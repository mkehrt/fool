#![allow(unused)]

use crate::color::Color;
use crate::{Context, HEIGHT, HEIGHT_IN_PIXELS, UNIT_IN_PIXELS, WIDTH_IN_PIXELS};

const MARGIN_IN_PIXELS: f64 = 36.0;

pub fn draw(context: &mut Context, with_inner: bool) {
    let margin_color = "rgba(255, 255, 255, 0.75)";
    context.set_fill_style(&margin_color.into());

    context.fill_rect(0.0, 0.0, WIDTH_IN_PIXELS, MARGIN_IN_PIXELS);
    context.fill_rect(
        0.0,
        HEIGHT_IN_PIXELS - MARGIN_IN_PIXELS,
        WIDTH_IN_PIXELS,
        MARGIN_IN_PIXELS,
    );
    context.fill_rect(
        0.0,
        MARGIN_IN_PIXELS,
        MARGIN_IN_PIXELS,
        HEIGHT_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
    );
    context.fill_rect(
        WIDTH_IN_PIXELS - MARGIN_IN_PIXELS,
        MARGIN_IN_PIXELS,
        MARGIN_IN_PIXELS,
        HEIGHT_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
    );

    let scale_and_translate_factors = vec![
        ((1.0, 1.0), (0.0, 0.0)),
        ((-1.0, 1.0), (WIDTH_IN_PIXELS, 0.0)),
        ((-1.0, -1.0), (WIDTH_IN_PIXELS, HEIGHT_IN_PIXELS)),
        ((1.0, -1.0), (0.0, HEIGHT_IN_PIXELS)),
    ];

    for ((scale_x, scale_y), (translate_x, translate_y)) in scale_and_translate_factors {
        context.save();

        context.translate(translate_x, translate_y);
        context.scale(scale_x, scale_y);

        context.move_to(MARGIN_IN_PIXELS, HEIGHT_IN_PIXELS - MARGIN_IN_PIXELS);
        context.begin_path();
        context.line_to(MARGIN_IN_PIXELS, HEIGHT_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS);
        context.arc_to(
            MARGIN_IN_PIXELS,
            HEIGHT_IN_PIXELS - MARGIN_IN_PIXELS,
            2.0 * MARGIN_IN_PIXELS,
            HEIGHT_IN_PIXELS - MARGIN_IN_PIXELS,
            0.70710678118 * MARGIN_IN_PIXELS,
        );
        context.line_to(MARGIN_IN_PIXELS, HEIGHT_IN_PIXELS - MARGIN_IN_PIXELS);
        context.close_path();

        context.fill();

        context.restore();
    }

    if (with_inner) {
        let inner_margin_color = "rgba(255, 255, 255, 0.25)";
        context.set_fill_style(&inner_margin_color.into());

        context.fill_rect(
            MARGIN_IN_PIXELS,
            MARGIN_IN_PIXELS,
            WIDTH_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
            MARGIN_IN_PIXELS,
        );
        context.fill_rect(
            MARGIN_IN_PIXELS,
            HEIGHT_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
            WIDTH_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
            MARGIN_IN_PIXELS,
        );
        context.fill_rect(
            MARGIN_IN_PIXELS,
            2.0 * MARGIN_IN_PIXELS,
            MARGIN_IN_PIXELS,
            HEIGHT_IN_PIXELS - 4.0 * MARGIN_IN_PIXELS,
        );
        context.fill_rect(
            WIDTH_IN_PIXELS - 2.0 * MARGIN_IN_PIXELS,
            2.0 * MARGIN_IN_PIXELS,
            MARGIN_IN_PIXELS,
            HEIGHT_IN_PIXELS - 4.0 * MARGIN_IN_PIXELS,
        );
    }
}
