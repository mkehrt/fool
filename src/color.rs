use wasm_bindgen::JsValue;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub static SKY: Color = Color::new(0, 0, 50);
    pub static SKY_STARS: Color = Color::new(220, 220, 220);
    pub static FOOL: Color = Color::new(255, 200, 0);

    pub static ROCK_LIGHT: Color = Color::new(110, 110, 110);
    pub static ROCK_MEDIUM: Color = Color::new(90, 90, 90);
    pub static ROCK_DARK: Color = Color::new(70, 70, 70);
    pub static ROCK_STROKE: Color = Color::new(25, 25, 25);
}
