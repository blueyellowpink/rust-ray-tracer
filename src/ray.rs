use crate::{
    color::Color,
    hit::Hit,
    object::Sphere,
    vec::{Point3D, Vec3D},
    World,
};

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &World) -> Color {
        if let Some(hit_record) = world.hit(self, 0.0, f64::INFINITY) {
            return Color::RGB(0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0)));
        }

        let unit_direction: Vec3D = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color_vector = (1.0 - t) * Color::White.to_vec3d() + t * Color::new(0.5, 0.7, 1.0);
        Color::RGB(color_vector)
    }
}
