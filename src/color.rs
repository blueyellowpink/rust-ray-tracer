use std::fmt;

#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "255 0 0"),
            Color::Green => write!(f, "0 255 0"),
            Color::Blue => write!(f, "0 0 255"),
        }
    }
}
