#![allow(dead_code, non_snake_case)]

use std::{time::Instant};

use draw::{color::Color, light::PointLight};
use math::{matrix::Matrix, tuples::Tuple};
use scene::{
    camera::{render, view_transform, Camera},
    world::World,
};

use crate::{
    draw::patterns::{Checkered, Solid},
    shapes::{plane::Plane, sphere::Sphere},
};

mod draw;
mod math;
mod scene;
mod shapes;
fn main() {
    let mut floor = Plane::new(Some(
        &Matrix::translation(0., 0., 10.) * &Matrix::rotation_x(1.5708),
    ));
    floor.material.pattern = Box::new(Checkered::new(
        Color::new(0.15, 0.15, 0.15),
        Color::new(0.85, 0.85, 0.85),
    ));
    floor.material.ambient = 0.8;
    floor.material.diffuse = 0.2;
    floor.material.specular = 0.0;

    let mut g_sphere = Sphere::new(None);
    g_sphere.material.pattern = Box::new(Solid::new(Color::white()));
    g_sphere.material.ambient = 0.0;
    g_sphere.material.diffuse = 0.0;
    g_sphere.material.specular = 0.9;
    g_sphere.material.shininess = 300.0;
    g_sphere.material.reflective = 0.9;
    g_sphere.material.transparency = 0.9;
    g_sphere.material.refractive_index = 1.5;

    let mut a_sphere = Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5)));
    a_sphere.material.pattern = Box::new(Solid::new(Color::white()));
    a_sphere.material.ambient = 0.0;
    a_sphere.material.diffuse = 0.0;
    a_sphere.material.specular = 0.9;
    a_sphere.material.shininess = 300.;
    a_sphere.material.reflective = 0.9;
    a_sphere.material.transparency = 0.9;
    a_sphere.material.refractive_index = 1.0000034;

    let mut world = World::new();
    world.objects = vec![Box::new(floor), Box::new(g_sphere), Box::new(a_sphere)];
    world.light_sources = vec![PointLight::new(
        Color::new(0.9, 0.9, 0.9),
        Tuple::point(2.0, 10.0, -5.0),
    )];

    let camera = Camera::new_with_transform(
        1500,
        1500,
        0.45,
        view_transform(
            Tuple::point(0.0, 0.0, -5.),
            Tuple::point(0.0, 0.0, 0.0),
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
