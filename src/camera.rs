use crate::{
    color::Color,
    ray::Ray,
    vec::{Point2D, Point3D, Vec3D},
};

pub struct Camera {
    pub origin: Point3D,
    pub lower_left_corner: Point3D,
    pub horizontal: Vec3D,
    pub vertical: Vec3D,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

impl Camera {
    pub fn new(
        origin: Point3D,
        focal_length: f64,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Self {
        let horizontal = Vec3D::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3D::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            focal_length,
            viewport_width,
            viewport_height,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
