use std::ops::{Add, Div, Mul, Range, Sub};

use rand::Rng;

use crate::color::Color;

pub type Point3D = Vec3D;

#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3D {
        self / self.length()
    }

    pub fn scale(self, factor: f64) -> Vec3D {
        self * factor
    }

    pub fn format_color(&self, samples_per_pixel: u64) -> Vec3D {
        Vec3D {
            x: 255.999
                * (self.x / (samples_per_pixel as f64))
                    .sqrt()
                    .clamp(0.0, 0.999),
            y: 255.999
                * (self.y / (samples_per_pixel as f64))
                    .sqrt()
                    .clamp(0.0, 0.999),
            z: 255.999
                * (self.z / (samples_per_pixel as f64))
                    .sqrt()
                    .clamp(0.0, 0.999),
        }
    }

    pub fn random(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range.clone()),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut v = Self::random(-1.0..1.0);
        while v.length() >= 1.0 {
            v = Self::random(-1.0..1.0);
        }
        v
    }

    pub fn random_in_hemisphere(normal: Vec3D) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        /* loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        } */
        let mut p = Vec3D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        while p.length() >= 1.0 {
            p = Vec3D::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        }
        p
    }

    pub fn is_near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn reflect(self, normal: Vec3D) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, normal: Vec3D, etai_over_etat: f64) -> Self {
        let cos_theta = ((-1.0) * self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'this, 'other> Add<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn add(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'this, 'other> Sub<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn sub(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3D {
    type Output = Vec3D;

    fn mul(self, other: Self) -> Vec3D {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'this, 'other> Mul<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn mul(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<'other> Mul<&'other Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<'this> Mul<f64> for &'this Vec3D {
    type Output = Vec3D;

    fn mul(self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<'this> Div<f64> for &'this Vec3D {
    type Output = Vec3D;

    fn div(self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_3d_dot_product_borrow() {
        let a = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vec3D {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            &a + &b,
            Vec3D {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        assert_eq!(
            &a - &b,
            Vec3D {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        assert_eq!(
            &a * &b,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            &a * 2.0,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            &b / 2.0,
            Vec3D {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn test_point_3d_dot_product_move() {
        let a = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vec3D {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            a + b,
            Vec3D {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        assert_eq!(
            a - b,
            Vec3D {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        assert_eq!(
            a * b,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            a * 2.0,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            b / 2.0,
            Vec3D {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }
}
