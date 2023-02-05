pub mod camera;
pub mod color;
pub mod ray;
pub mod vec;

pub mod hit {
    use crate::{
        ray::Ray,
        vec::{Point3D, Vec3D},
    };

    pub struct HitRecord {
        pub hit_point: Point3D,
        pub normal: Vec3D,
        pub t: f64,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn new(hit_point: Point3D, normal: Vec3D, t: f64, ray: &Ray) -> Self {
            let front_face = ray.direction.dot(normal) < 0.0;
            let normal = if front_face { normal } else { -1.0 * normal };
            Self {
                hit_point,
                normal,
                t,
                front_face,
            }
        }
    }

    pub trait Hit {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    }
}

pub mod object {
    use crate::{
        hit::{Hit, HitRecord},
        ray::Ray,
        vec::Point3D,
    };

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Sphere {
        pub center: Point3D,
        pub radius: f64,
    }

    impl Sphere {
        pub fn new(center: Point3D, radius: f64) -> Self {
            Self { center, radius }
        }
    }

    impl Hit for Sphere {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
            let oc = ray.origin - self.center;
            let a = ray.direction.length().powi(2);
            let half_b = oc.dot(ray.direction);
            let c = oc.length().powi(2) - self.radius.powi(2);
            let discriminant = half_b.powi(2) - a * c;

            if discriminant < 0.0 {
                return None;
            }

            let sqrt_discriminant = discriminant.sqrt();
            let mut t = (-half_b - sqrt_discriminant) / a;
            if t < t_min || t_max < t {
                t = (-half_b + sqrt_discriminant) / a;
                if t < t_min || t_max < t {
                    return None;
                }
            }
            let hit_point = ray.at(t);
            let normal = (hit_point - self.center) / self.radius;
            let hit_record = HitRecord::new(hit_point, normal, t, ray);

            Some(hit_record)
        }
    }
}

use hit::{Hit, HitRecord};
use ray::Ray;

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                temp_rec = Some(hit_record);
            }
        }

        temp_rec
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiffuseRenderer {
    LambertianReflection,
    HemisphericalScattering,
}
