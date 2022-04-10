#![allow(dead_code, non_snake_case)]

use draw::{canvas::Canvas, color::Color};
use math::{matrix::Matrix, ray::Ray, tuples::Tuple};
use shapes::{intersect::Intersect, sphere::Sphere};

mod draw;
mod math;
mod shapes;
fn main() {
    let mut c = Canvas::new(1000, 1000);

    let sphere_transform = &Matrix::translation(500., 500., 0.) * &Matrix::scaling(100., 100., 1.0);

    let sphere = Sphere::new(Some(sphere_transform));

    for x in 0..1000 {
        for y in 0..1000 {
            let ray = Ray::new(
                Tuple::point(x as f32, y as f32, 0.0),
                Tuple::vector(0.0, 0.0, -1.0),
            );
            if !sphere.intersect(&ray).is_empty() {
                c.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
            } else {
                c.write_pixel(x, y, Color::new(0.0, 0.0, 0.0));
            }
        }
        println!("Finished line {} out of 1000", x);
    }

    c.write_to_ppm();
}
