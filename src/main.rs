#![allow(
    dead_code,
    non_snake_case
  )]

use canvas::Canvas;
use color::Color;

mod canvas;
mod color;
mod tuples;
mod utils;
mod matrix;

fn main() {
    let mut c = Canvas::new(20, 10);

    let red = Color::new(1.0, 0.0, 0.0);
    let green = Color::new(0.0, 1.0, 0.0);

    for i in 0..c.height {
        for j in 0..c.width / 2 {
            c.write_pixel(i, j, red);
        }
        for j in c.width / 2..c.width {
            c.write_pixel(i, j, green);
        }
    }

    c.write_to_ppm();
}
