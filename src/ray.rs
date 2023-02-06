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

    pub fn color(&self, world: &World, ray_bounce_depth: usize) -> Color {
        if ray_bounce_depth <= 0 {
            return Color::Black;
        }

        if let Some(hit_record) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered_ray)) =
                hit_record.material.scatter(self, &hit_record)
            {
                return Color::RGB(
                    attenuation.to_vec3d()
                        * scattered_ray.color(world, ray_bounce_depth - 1).to_vec3d(),
                );
            } else {
                return Color::Black;
            }
        }

        let unit_direction: Vec3D = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color_vector =
            (1.0 - t) * Color::White.to_vec3d() + t * Color::new(0.5, 0.7, 1.0).to_vec3d();
        Color::RGB(color_vector)
    }
}
