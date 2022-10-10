use crate::generate_perturbable;
use crate::perturbable::Perturbable;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: Red,
    pub green: Green,
    pub blue: Blue,
}

impl Color {
   pub fn perturb(&self) -> Self {
       let red = self.red.perturb();
       let green = self.green.perturb();
       let blue = self.blue.perturb();

       Self { red, blue, green}
    }

   pub fn new_random() -> Self {
       let red = Red::new_random();
       let green = Green::new_random();
       let blue = Blue::new_random();

       Self { red, green, blue }
    }
}

generate_perturbable!(i64, Red, 0, 255, 5);
generate_perturbable!(i64, Green, 0, 255, 5);
generate_perturbable!(i64, Blue, 0, 255, 5);

/*
use std::convert::From;
use wasm_bindgen::prelude::*;

impl From<Color> for JsValue {
   fn from(that: Color) -> JsValue {
      JSVal
   }
}
*/
