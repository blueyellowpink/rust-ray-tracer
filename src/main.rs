use rust_ray_tracer::{color::Color, point::Point2D};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

type Grid = [[u8; WIDTH]; HEIGHT];

struct Image {
    pub inner: Grid,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            inner: [[0u8; WIDTH]; HEIGHT],
            width,
            height,
        }
    }

    fn fill_center_circle(&mut self, radius: usize) {
        let origin = Point2D {
            x: (self.width / 2) as isize,
            y: (self.height / 2) as isize,
        };

        let calc_distance = |x: isize, y: isize| -> usize {
            ((x - origin.x).pow(2) + (y - origin.y).pow(2))
                .try_into()
                .unwrap()
        };

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if calc_distance(x as isize, y as isize) < radius.pow(2) {
                    self.inner[y][x] = 1;
                }
            }
        }
    }

    fn write_ppm_stdout(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.inner[y][x] == 1 {
                    println!("{}", Color::Red);
                    continue;
                }
                println!("{}", Color::Green);
            }
        }
    }
}

fn main() {
    let mut image = Image::new(WIDTH, HEIGHT);
    image.fill_center_circle(50);
    image.write_ppm_stdout();
}
