use std::fmt;

use crate::vec::Vec3D;

#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
    RGB(Vec3D),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "255 0 0"),
            Color::Green => write!(f, "0 255 0"),
            Color::Blue => write!(f, "0 0 255"),
            Color::Black => write!(f, "0 0 0"),
            Color::RGB(vec3) => write!(f, "{} {} {}", vec3.x, vec3.y, vec3.z),
        }
    }
}
