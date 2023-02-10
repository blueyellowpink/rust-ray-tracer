use rand::Rng;

use crate::{color::Color, hit::HitRecord, ray::Ray, vec::Vec3D};

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3D::random_in_unit_sphere().normalize();
        if scatter_direction.is_near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = hit_record.normal;
        }
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);

        Some((self.albedo, scattered_ray))
    }
}

pub struct Hemisphere {
    albedo: Color,
}

impl Hemisphere {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Hemisphere {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction =
            hit_record.hit_point + Vec3D::random_in_hemisphere(hit_record.normal);
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            scatter_direction - hit_record.hit_point,
        );

        Some((self.albedo, scattered_ray))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_vector = ray_in.direction.reflect(hit_record.normal).normalize();
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            reflected_vector + self.fuzz * Vec3D::random_in_unit_sphere(),
        );

        if scattered_ray.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = ((-1.0) * unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };
        let scattered_ray = Ray::new(hit_record.hit_point, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered_ray))
    }
}
