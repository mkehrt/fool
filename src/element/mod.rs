use crate::color::Color;
use crate::Context;
use crate::generate_perturbable;
use crate::perturbable::Perturbable;

mod children;
mod draw;

use children::Children;

#[derive(Debug)]
pub struct Element {
    pub distal_color: Color,
    pub height: Height,
    pub angle: Angle,
    pub children: Children,
}

impl Element {
    pub fn draw(&self, parent: &Parent, context: &mut Context) {
        draw::draw(context, self, parent)
    }

    pub fn perturb(&self) -> Self {
        let distal_color = self.distal_color.perturb();
        let height = self.height.perturb();
        let angle = self.angle.perturb();
        let children = self.children.perturb();

        Self { distal_color, height, angle, children }
    }

    pub fn new_random() -> Self {
        let distal_color = Color::new_random();
        let height = Height::new_random();
        let angle = Angle::new_random();
        let children = Children::new();
    
        Self { distal_color, height, angle, children }
    }
}

generate_perturbable!(f64, Height, 0.1, 5.0, 0.3);
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
        Self{ color }
    }
}
