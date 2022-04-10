#![allow(
    dead_code,
    non_snake_case
  )]

use std::f32::consts::PI;

use canvas::Canvas;
use color::Color;
use matrix::Matrix;
use tuples::Tuple;

mod canvas;
mod color;
mod tuples;
mod utils;
mod matrix;
mod ray;

fn main() {
    let mut c = Canvas::new(100, 100);

    let black = Color::new(0.0, 0.0, 0.0);

    let origin = Tuple::point(50., 50., 0.0);
    let mut hand = Tuple::vector(0.0, 25.0, 0.0);

    let rotation = Matrix::rotation_z(PI / 6.0);

    for _ in 0..12 {
        let cur = origin + hand;
        c.write_pixel(cur.x as usize, cur.y as usize, black);
        hand = &rotation * &hand;
    }

    c.write_to_ppm();
}
