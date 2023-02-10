use std::rc::Rc;

use rand::Rng;

use rust_ray_tracer::{
    camera::Camera,
    color::Color,
    material::{Dielectric, Hemisphere, Lambertian, Metal},
    object::Sphere,
    ray::Ray,
    vec::{Point2D, Point3D, Vec3D},
    World,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 256;
const HEIGHT: usize = 144;
const SAMPLES_PER_PIXEL: usize = 50;
const MAX_RAY_BOUNCE_DEPTH: usize = 5;
const ANTI_ALIAS: bool = true;

trait RayTraceable {
    fn trace_to_ppm_with(&self, camera: Camera, world: World);
}

type Grid = [[u8; WIDTH]; HEIGHT];

struct Image {
    pub inner: Grid,
    pub width: usize,
    pub height: usize,
    pub aspect_ratio: f64,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            inner: [[0u8; WIDTH]; HEIGHT],
            width,
            height,
            aspect_ratio: (width as f64) / (height as f64),
        }
    }

    fn fill_center_circle(&mut self, radius: f64) {
        let origin = Point2D {
            x: (self.width / 2) as f64,
            y: (self.height / 2) as f64,
        };

        let calc_distance =
            |x: f64, y: f64| -> f64 { (x - origin.x).powf(2.0) + (y - origin.y).powf(2.0) };

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if calc_distance(x as f64, y as f64) < radius.powf(2.0) {
                    self.inner[y][x] = 1;
                }
            }
        }
    }

    fn write_ppm_stdout(&self, fg: Color, bg: Color) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.inner[y][x] == 1 {
                    println!("{}", fg);
                    continue;
                }
                println!("{}", bg);
            }
        }
    }

    fn fill_intersect(&mut self) {
        let ray_intersect_sphere = |_x: f64, _y: f64| -> bool { false };

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if ray_intersect_sphere(x as f64, y as f64) {
                    self.inner[y][x] = 1;
                }
            }
        }
    }
}

impl RayTraceable for Image {
    fn trace_to_ppm_with(&self, camera: Camera, world: World) {
        let get_uv = |x, y, random_u, random_v| -> (f64, f64) {
            let u: f64 = ((x as f64) + random_u) / ((self.width - 1) as f64);
            let v: f64 = ((y as f64) + random_v) / ((self.height - 1) as f64);
            (u, v)
        };

        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        let mut rng = rand::thread_rng();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if ANTI_ALIAS {
                    let mut sum_color: Vec3D = Color::new(0.0, 0.0, 0.0).to_vec3d();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let (u, v) = get_uv(x, y, rng.gen(), rng.gen());
                        let ray = camera.get_ray(u, v);
                        let color = ray.color(&world, MAX_RAY_BOUNCE_DEPTH).to_vec3d();
                        sum_color = sum_color + color;
                    }
                    println!(
                        "{}",
                        Color::RGB(sum_color.format_color(SAMPLES_PER_PIXEL as u64))
                    );
                } else {
                    let (u, v) = get_uv(x, y, 0.0, 0.0);
                    let ray = camera.get_ray(u, v);
                    let color = ray.color(&world, MAX_RAY_BOUNCE_DEPTH).to_vec3d();
                    println!("{}", Color::RGB(color.format_color(1)));
                }
            }
        }
    }
}

fn a() {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_left_inner = Rc::new(Dielectric::new(1.1));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = World::new();
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    /* world.push(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ))); */
    world.push(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left_inner,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let image = Image::new(WIDTH, HEIGHT);
    let camera = Camera::new(
        Point3D::new(-2.0, 2.0, 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        Vec3D::new(0.0, 1.0, 0.0),
        20.0,
        1.0,
        ASPECT_RATIO,
    );
    image.trace_to_ppm_with(camera, world);
}

fn b() {
    let r: f64 = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sphere_left = Sphere::new(Point3D::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3D::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    /* let image = Image::new(WIDTH, HEIGHT);
    let camera = Camera::new(90.0, 1.0, ASPECT_RATIO);
    image.trace_to_ppm_with(camera, world); */
}

fn main() {
    a();
    // b();
}
