#![allow(dead_code, non_snake_case)]

use clap::{App, Arg};
use scene::camera::render;

mod draw;
mod examples;
mod math;
mod obj_parser;
mod scene;
mod shapes;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("An experimental ray tracer")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("THREADS")
                .help("The number of threads used to render the images")
                .default_value("6")
                .takes_value(true),
        )
        .get_matches();

    let threads = match matches.value_of("threads").unwrap().parse::<usize>() {
        Ok(t) => t,
        Err(_) => {
            println!("Invalid number of threads");
            return;
        }
    };

    let scene = examples::pawn_chess();

    let image = render(scene.0, scene.1, threads);
    image.write_to_ppm("canvas.ppm");
}
