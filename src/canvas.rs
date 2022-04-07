use std::fs::File;
use std::io::Write;

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            canvas: vec![vec![Color::white(); width]; height],
        }
    }

    pub fn write_pixel(&mut self, i: usize, j: usize, c: Color) {
        self.canvas[i][j] = c;
    }

    pub fn get_pixel(&self, i: usize, j: usize) -> Color {
        self.canvas[i][j]
    }

    pub fn write_to_ppm(&self) {
        let mut file = File::create("canvas.ppm").expect("could not create file");

        // file header
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut file, "255").unwrap();

        for i in 0..self.height {
            for j in 0..self.width {
                write!(&mut file, "{} ", self.get_pixel(i, j)).unwrap();
            }
            writeln!(&mut file).unwrap();
        }
    }
}
