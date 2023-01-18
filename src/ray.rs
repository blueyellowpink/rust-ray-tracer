use crate::vec::{Point3D, Vec3D};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3D,
}
