#![allow(dead_code, non_snake_case)]

use std::{f64::consts::PI, time::Instant};

use draw::{color::Color, light::PointLight};
use math::{matrix::Matrix, tuples::Tuple};
use scene::{
    camera::{render, view_transform, Camera},
    world::World,
};

use crate::{
    draw::patterns::{Solid, Stripe},
    shapes::{plane::Plane, sphere::Sphere},
};

mod draw;
mod math;
mod scene;
mod shapes;
fn main() {
    let mut middle = Sphere::new(Some(Matrix::translation(-0.5, 1.0, 0.5)));
    middle.material.pattern = Box::new(Stripe::new(Color::new(0.1, 1.0, 0.5), Color::white()));
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new(Some(
        &Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5),
    ));
    right.material.pattern = Box::new(Solid::new(Color::new(0.5, 1.0, 0.1)));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new(Some(
        &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
    ));
    left.material.pattern = Box::new(Solid::new(Color::new(1.0, 0.8, 0.1)));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let floor = Plane::new(None);

    let mut world = World::new();
    world.objects = vec![
        Box::new(left),
        Box::new(middle),
        Box::new(right),
        Box::new(floor),
    ];
    world.light_sources = vec![PointLight::new(
        Color::new(1.0, 1.0, 1.0),
        Tuple::point(-10.0, 10.0, -10.),
    )];

    let camera = Camera::new_with_transform(
        1000,
        500,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let start = Instant::now();
    let image = render(camera, world, 6);
    image.write_to_ppm("canvas.ppm");
    println!(
        "Image rendering finished in {}s. File written to canvas.ppm",
        Instant::now().duration_since(start).as_secs()
    );
}
