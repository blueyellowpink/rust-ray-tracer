use rust_ray_tracer::{
    color::Color,
    ray::Ray,
    vec::{Point2D, Point3D, Vec3D},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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

struct Camera {
    position: Point3D,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
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

fn main() {
    let image = Image::new(WIDTH, HEIGHT);
    let origin = Point3D::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let camera = Camera::new(
        origin,
        1.0,
        image.aspect_ratio * viewport_height,
        viewport_height,
    );
    let horizontal = Vec3D::new(camera.viewport_width, 0.0, 0.0);
    let vertical = Vec3D::new(0.0, camera.viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3D::new(0.0, 0.0, camera.focal_length);

    println!("P3");
    println!("{} {}", image.width, image.height);
    println!("255");
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let u: f64 = (x as f64) / ((WIDTH - 1) as f64);
            let v: f64 = (y as f64) / ((HEIGHT - 1) as f64);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray.color();
            // println!("{}", color);
        }
    }
}
