use std::fmt;

use crate::vec::Vec3D;

#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
    RGB(Vec3D),
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Vec3D {
        Vec3D::new(red, green, blue)
    }

    pub fn to_vec3d(&self) -> Vec3D {
        match self {
            Color::Red => Vec3D::new(1.0, 0.0, 0.0),
            Color::Green => Vec3D::new(0.0, 1.0, 0.0),
            Color::Blue => Vec3D::new(0.0, 0.0, 1.0),
            Color::Black => Vec3D::new(0.0, 0.0, 0.0),
            Color::White => Vec3D::new(1.0, 1.0, 1.0),
            Color::RGB(vec3) => *vec3,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::RGB(vec3) => {
                let vec3 = vec3.format_color();
                write!(
                    f,
                    "{} {} {}",
                    vec3.x as usize, vec3.y as usize, vec3.z as usize
                )
            }
            _ => write!(f, "{}", Color::RGB(self.to_vec3d())),
        }
    }
}
