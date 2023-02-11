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
    pub cu: Vec3D,
    pub cv: Vec3D,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3D,
        lookat: Vec3D,
        view_up: Vec3D,
        vertical_field_of_view: f64,
        focal_length: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vertical_field_of_view;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalize();
        let cu = view_up.cross(cw).normalize();
        let cv = cw.cross(cu);

        let horizontal = focus_distance * viewport_width * cu;
        let vertical = focus_distance * viewport_height * cv;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_distance * cw;

        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            focal_length,
            cu,
            cv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3D::random_in_unit_disk();
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
