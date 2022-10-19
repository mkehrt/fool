use crate::color::Color;
use crate::generate_perturbable;
use crate::perturbable::Perturbable;
use crate::Context;

use num::Float;

mod children;
mod draw;

use children::Children;

#[derive(Debug)]
pub struct Element {
    pub distal_color: Color,
    height_exponent: HeightExponent,
    angle: Angle,
    pub children: Children,
}

impl Element {
    pub fn draw(&self, parent: &Parent, context: &mut Context) {
        draw::draw(
            context, /* depth = */ 1, /* length_scale = */ 1.0, self, parent,
        )
    }

    pub fn perturb(&self) -> Self {
        let distal_color = self.distal_color.perturb();
        let height_exponent = self.height_exponent.perturb();
        let angle = self.angle.perturb();
        let children = self.children.perturb();

        Self {
            distal_color,
            height_exponent,
            angle,
            children,
        }
    }

    pub fn new_random() -> Self {
        let distal_color = Color::new_random();
        let height_exponent = HeightExponent::new_random();
        let angle = Angle::new_random();
        let children = Children::new();

        Self {
            distal_color,
            height_exponent,
            angle,
            children,
        }
    }

    pub fn height(&self) -> f64 {
        Float::powf(3.0, self.height_exponent.value)
    }

    fn angle(&self) -> f64 {
        self.angle.value * std::f64::consts::PI / 180.0
    }

    pub fn height_statistics(&self) -> (i64, f64) {
        let mut count = 1;
        let mut total_height = self.height();

        for child in self.children.children.iter() {
            let (count_delta, total_height_delta) = child.height_statistics();
            count += count_delta;
            total_height += total_height_delta;
        }

        (count, total_height)
    }
}

generate_perturbable!(f64, HeightExponent, -1.0, 1.0, 0.3);
generate_perturbable!(f64, Angle, -90.0, 90.0, 5.0);

pub struct Parent {
    pub color: Color,
}

impl Parent {
    pub fn perturb(&self) -> Self {
        let color = self.color.perturb();
        Self { color }
    }

    pub fn new_random() -> Self {
        let color = Color::new_random();
        Self { color }
    }
}
