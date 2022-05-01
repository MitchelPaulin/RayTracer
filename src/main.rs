#![allow(dead_code, non_snake_case)]

use std::f32::consts::PI;

use draw::{color::Color, light::PointLight};
use math::{matrix::Matrix, tuples::Tuple};
use scene::{
    camera::{view_transform, Camera},
    world::{World},
};
use shapes::{
    sphere::Sphere,
};

mod draw;
mod math;
mod scene;
mod shapes;
fn main() {
    let mut floor = Sphere::new(Some(Matrix::scaling(10.0, 0.01, 10.0)));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new(Some(
        &Matrix::translation(0.0, 0.0, 5.0)
            * &(&Matrix::rotation_y(-PI / 4.0)
                * &(&Matrix::rotation_x(PI / 2.0) * &Matrix::scaling(10.0, 0.01, 10.0))),
    ));
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new(Some(
        &Matrix::translation(0.0, 0.0, 5.0)
            * &(&Matrix::rotation_y(PI / 4.0)
                * &(&Matrix::rotation_x(PI / 2.0) * &Matrix::scaling(10.0, 0.01, 10.0))),
    ));
    right_wall.material = floor.material;

    let mut middle = Sphere::new(Some(Matrix::translation(-0.5, 1.0, 0.5)));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new(Some(
        &Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5),
    ));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new(Some(
        &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
    ));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.objects = vec![
        Box::new(floor),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(left),
        Box::new(middle),
        Box::new(right),
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
    let image = camera.render(&world);
    image.write_to_ppm();
}
