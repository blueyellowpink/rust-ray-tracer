use crate::{
    color::Color,
    vec::{Point3D, Vec3D},
};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let unit_direction: Vec3D = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color_vector = (1.0 - t) * Vec3D::new(1.0, 1.0, 1.0) + t * Vec3D::new(0.5, 0.7, 1.0);
        Color::RGB(color_vector.format_color())
    }
}
