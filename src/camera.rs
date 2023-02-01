use crate::{
    color::Color,
    ray::Ray,
    vec::{Point2D, Point3D, Vec3D},
};

pub struct Camera {
    pub position: Point3D,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

impl Camera {
    pub fn new(
        position: Point3D,
        focal_length: f64,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Self {
        Self {
            position,
            focal_length,
            viewport_width,
            viewport_height,
        }
    }
}
