#![allow(dead_code, non_snake_case)]

use std::{f64::consts::PI, time::Instant};

use draw::{color::Color, light::PointLight};
use math::{matrix::Matrix, tuples::Tuple};
use scene::{
    camera::{render, view_transform, Camera},
    world::World,
};

use crate::{
    draw::patterns::{Checkered, Rings, Solid},
    shapes::{plane::Plane, sphere::Sphere},
};

mod draw;
mod math;
mod scene;
mod shapes;
fn main() {
    let mut middle = Sphere::new(Some(Matrix::translation(-0.5, 1.0, 0.5)));
    middle.material.pattern = Box::new(Solid::new(Color::black()));
    middle.material.specular = 1.;
    middle.material.transparency = 1.0;
    middle.material.reflective = 0.9;
    middle.material.shininess = 300.;
    middle.material.ambient = 0.1;
    middle.material.diffuse = 0.1;
    middle.material.refractive_index = 1.333;

    let mut middle_behind = Sphere::new(Some(Matrix::translation(0.5, 1.0, 3.)));
    middle_behind.material.pattern = Box::new(Solid::new(Color::new(1.0, 0.0, 0.0)));
    middle_behind.material.diffuse = 0.7;
    middle_behind.material.specular = 0.3;
    middle_behind.material.reflective = 0.01;

    let mut right = Sphere::new(Some(
        &Matrix::translation(1.5, 0.5, -0.5)
            * &(&Matrix::scaling(0.5, 0.5, 0.5) * &Matrix::rotation_z(-PI / 3.0)),
    ));
    right.material.pattern = Box::new(Checkered::new(
        Color::new(0.461, 0.586, 0.336),
        Color::new(0.93, 0.93, 0.82),
    ));
    right
        .material
        .pattern
        .set_transform(Matrix::scaling(0.5, 0.5, 0.5));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.1;

    let mut left = Sphere::new(Some(
        &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
    ));
    left.material.pattern = Box::new(Rings::new(Color::new(1.0, 0.8, 0.1), Color::black()));
    left.material
        .pattern
        .set_transform(&Matrix::rotation_z(-PI / 3.0) * &Matrix::scaling(0.33, 0.33, 0.33));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.reflective = 0.1;

    let mut floor = Plane::new(None);
    floor.material.pattern = Box::new(Checkered::new(Color::black(), Color::white()));
    floor.material.reflective = 0.1;

    let mut ceil = Plane::new(Some(Matrix::translation(0., 100., 0.)));
    ceil.material.pattern = Box::new(Solid::new(Color::new(0., 0.707, 0.882)));
    ceil.material.specular = 1.;
    ceil.material.diffuse = 1.;
    ceil.material.ambient = 0.8;
    ceil.material.reflective = 0.3;

    let mut world = World::new();
    world.objects = vec![
        Box::new(left),
        Box::new(middle),
        Box::new(right),
        Box::new(floor),
        Box::new(ceil),
        Box::new(middle_behind)
    ];
    world.light_sources = vec![PointLight::new(
        Color::new(1.0, 1.0, 1.0),
        Tuple::point(-10.0, 10.0, -10.),
    )];

    let camera = Camera::new_with_transform(
        500,
        500,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 2.0, -5.0),
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
