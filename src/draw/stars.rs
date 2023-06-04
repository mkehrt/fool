use rand::Rng;

use crate::color::{colors, Color};
use crate::log::log;
use crate::{Context, HEIGHT, WIDTH};

use super::star;

const NUM_STARS: u64 = 180;
const MAX_STAR_RADIUS: f64 = 0.20;
const MIN_STAR_RADIUS: f64 = 0.05;
const STAR_MARGIN_RADIUS: f64 = 0.25;

pub fn draw<R: Rng>(context: &mut Context, mut rng: R) {
    let mut stars = 0;

    let min_star_radius_sqrt = f64::sqrt(MIN_STAR_RADIUS);
    let max_star_radius_sqrt = f64::sqrt(MAX_STAR_RADIUS);

    loop {
        if stars == NUM_STARS {
            break;
        }

        let star_x = rng.gen_range(0.0..WIDTH - MAX_STAR_RADIUS * 2.0) + MAX_STAR_RADIUS;
        let star_y = rng.gen_range(0.0..HEIGHT - MAX_STAR_RADIUS * 2.0) + MAX_STAR_RADIUS;
        let star_radius_sqrt = rng.gen_range(min_star_radius_sqrt..max_star_radius_sqrt);
        let star_radius = star_radius_sqrt * star_radius_sqrt;
        let star_angle = rng.gen_range(0.0..360.0);

        if !star_is_allowed(context, star_x, star_y) {
            log(format!("star: {:}, {:}, {:}", star_x, star_y, star_radius).as_str());
            continue;
        }

        star::draw(
            context,
            colors::SKY_STARS,
            star_x,
            star_y,
            star_radius,
            star_angle,
        );

        stars += 1;
    }

    // Draw some extra stars to fill in blank spots.
    // Inside fool.
    star::draw(context, colors::SKY_STARS, 12.25, 18.0, 0.17, 270.0);
    star::draw(context, colors::SKY_STARS, 13.25, 21.5, 0.12, 20.0);
    // Elsewhere
    star::draw(context, colors::SKY_STARS, 3.25, 3.5, 0.14, 27.0);
    star::draw(context, colors::SKY_STARS, 5.5, 8.0, 0.16, 53.0);
    star::draw(context, colors::SKY_STARS, 3.5, 31.0, 0.12, 70.0);
    star::draw(context, colors::SKY_STARS, 10.3, 9.4, 0.12, 23.0);
    star::draw(context, colors::SKY_STARS, 16.5, 30.0, 0.18, 51.0);
    star::draw(context, colors::SKY_STARS, 17.4, 31.0, 0.1, 86.0);
    star::draw(context, colors::SKY_STARS, 18.4, 37.0, 0.14, 44.0);
    star::draw(context, colors::SKY_STARS, 22.4, 32.0, 0.18, 33.0);
}

fn star_is_allowed(context: &mut Context, star_x: f64, star_y: f64) -> bool {
    let (center_x, center_y) = apply_current_transform(context, star_x, star_y);
    let (half_width, half_height) =
        apply_current_transform(context, STAR_MARGIN_RADIUS, STAR_MARGIN_RADIUS);

    let x = center_x - half_width;
    let y = center_y - half_height;

    let width = half_width * 2.0;
    let height = half_height * 2.0;

    log(format!("image chunk: {:}, {:}, {:}, {:}", x, y, width, height).as_str());

    let image_data = context
        .get_image_data(x, y, width, height)
        .expect("Get image data");
    let data = image_data.data().0;

    let pixels = data.chunks(4).map(|chunk| match chunk {
        [r, g, b, _a] => Color::new(*r, *g, *b),
        _ => panic!("Wrong number of elements in ImageData: {:?}", data),
    });

    for pixel in pixels {
        if pixel != colors::SKY {
            return false;
        }
    }

    // display_bounding_boxes(context, data.len(), x, y, width);

    true
}

fn apply_current_transform(context: &mut Context, x: f64, y: f64) -> (f64, f64) {
    let transform = context.get_transform().expect("Get transform");

    let a = transform.a();
    let b = transform.b();
    let c = transform.c();
    let d = transform.d();
    let e = transform.e();
    let f = transform.f();

    let x_out = a * x + b * y + e;
    let y_out = c * x + d * y + f;

    (x_out, y_out)
}

#[allow(unused)]
fn display_bounding_boxes(context: &mut Context, bytes: usize, x: f64, y: f64, width: f64) {
    let size = bytes / 4;
    let mut out_data = Vec::new();
    for _ in 0..size {
        out_data.append(&mut vec![100, 0, 100, 255]);
    }
    let out_image_data = web_sys::ImageData::new_with_u8_clamped_array(
        wasm_bindgen::Clamped(&out_data),
        width as u32,
    )
    .expect("data");
    context
        .put_image_data(&out_image_data, x, y)
        .expect("Put image data");
}
