use rust_ray_tracer::{
    camera::Camera,
    color::Color,
    object::Sphere,
    ray::Ray,
    vec::{Point2D, Point3D, Vec3D},
    World,
};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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
        let horizontal = Vec3D::new(camera.viewport_width, 0.0, 0.0);
        let vertical = Vec3D::new(0.0, camera.viewport_height, 0.0);
        let lower_left_corner = camera.position
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3D::new(0.0, 0.0, camera.focal_length);

        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let u: f64 = (x as f64) / ((self.width - 1) as f64);
                let v: f64 = (y as f64) / ((self.height - 1) as f64);
                let ray = Ray::new(
                    camera.position,
                    lower_left_corner + u * horizontal + v * vertical - camera.position,
                );
                let color = ray.color(&world);
                println!("{}", color);
            }
        }
    }
}

fn main() {
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let image = Image::new(WIDTH, HEIGHT);
    let origin = Point3D::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let camera = Camera::new(
        origin,
        1.0,
        image.aspect_ratio * viewport_height,
        viewport_height,
    );
    image.trace_to_ppm_with(camera, world);
}
