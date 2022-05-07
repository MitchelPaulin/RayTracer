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
        self.canvas[y][x] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.canvas[y][x]
    }

    pub fn write_to_ppm(&self, file_name: &str) {
        let mut file = File::create(file_name).expect("could not create file");

        // file header
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut file, "255").unwrap();

        for y in 0..self.height {
            let mut builder = Builder::default();
            for x in 0..self.width {
                builder.append(self.get_pixel(x, y).to_string() + " ");
            }
            writeln!(&mut file, "{}", builder.string().unwrap()).unwrap();
        }
    }
}

pub fn stitch_canvases(canvases: Vec<Canvas>) -> Canvas {
    assert!(!canvases.is_empty());
    let width = canvases[0].width;
    let height = canvases.iter().map(|c| c.height).sum();
    let mut result = Canvas::new(width, height);
    let mut res_y = 0;

    for canvas in canvases {
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                result.write_pixel(x, res_y, canvas.get_pixel(x, y));
            }
            res_y += 1;
        }
    }

    result
}
