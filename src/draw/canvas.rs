use std::fs::File;
use std::io::Write;
use string_builder::Builder;

use super::color::Color;

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
            canvas: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[x][y] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.canvas[x][y]
    }

    pub fn write_to_ppm(&self) {
        let mut file = File::create("canvas.ppm").expect("could not create file");

        // file header
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut file, "255").unwrap();

        for i in 0..self.height {
            let mut builder = Builder::default();
            for j in 0..self.width {
                builder.append(self.get_pixel(i, j).to_string() + " ");
            }
            writeln!(&mut file, "{}", builder.string().unwrap()).unwrap();
        }
    }
}
