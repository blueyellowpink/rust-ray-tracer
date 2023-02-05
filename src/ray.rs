use crate::{
    color::Color,
    hit::Hit,
    object::Sphere,
    vec::{Point3D, Vec3D},
    DiffuseRenderer, World,
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

    pub fn color(
        &self,
        world: &World,
        ray_bounce_depth: usize,
        diffuse_renderer: DiffuseRenderer,
    ) -> Color {
        if ray_bounce_depth <= 0 {
            return Color::Black;
        }

        if let Some(hit_record) = world.hit(self, 0.001, f64::INFINITY) {
            let target = match diffuse_renderer {
                DiffuseRenderer::LambertianReflection => {
                    hit_record.hit_point
                        + hit_record.normal
                        + Vec3D::random_in_unit_sphere().normalize()
                }
                DiffuseRenderer::HemisphericalScattering => {
                    hit_record.hit_point + Vec3D::random_in_hemisphere(hit_record.normal)
                }
            };
            let ray = Self::new(hit_record.hit_point, target - hit_record.hit_point);
            return Color::RGB(
                0.5 * ray
                    .color(world, ray_bounce_depth - 1, diffuse_renderer)
                    .to_vec3d(),
            );
        }

        let unit_direction: Vec3D = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color_vector = (1.0 - t) * Color::White.to_vec3d() + t * Color::new(0.5, 0.7, 1.0);
        Color::RGB(color_vector)
    }
}
