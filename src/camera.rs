use crate::{
    color::Color,
    ray::Ray,
    vec::{Point3D, Vec3D},
};

pub struct Camera {
    pub origin: Point3D,
    pub lower_left_corner: Point3D,
    pub horizontal: Vec3D,
    pub vertical: Vec3D,
    pub focal_length: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3D,
        lookat: Vec3D,
        view_up: Vec3D,
        vertical_field_of_view: f64,
        focal_length: f64,
        aspect_ratio: f64,
    ) -> Self {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vertical_field_of_view;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalize();
        let cu = view_up.cross(cw).normalize();
        let cv = cw.cross(cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - cw;

        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            focal_length,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
