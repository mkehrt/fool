use wasm_bindgen::JsValue;

#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<Color> for JsValue {
    fn from(that: Color) -> JsValue {
        let str = format!("rgb({:}, {:}, {:})", that.red, that.green, that.blue);
        let string = String::from(str);
        string.into()
    }
}

pub mod colors {
    use super::Color;

    pub static SKY: Color = Color::new(0, 00, 50);
    pub static SKY_STARS: Color = Color::new(220, 220, 220);
    pub static ROCK: Color = Color::new(100, 100, 100);
}
