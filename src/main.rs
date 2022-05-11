#![allow(dead_code, non_snake_case)]

use std::{f64::consts::PI, time::Instant};

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
    let mut floor = Plane::new(Some(Matrix::translation(0., -10., 0.)));
    floor.material.pattern = Box::new(Checkered::new(Color::black(), Color::white()));

    let mut g_sphere = Sphere::new(None);
    g_sphere.material.pattern = Box::new(Solid::new(Color::new(0., 0., 0.0)));
    g_sphere.material.diffuse = 0.1;
    g_sphere.material.shininess = 300.;
    g_sphere.material.reflective = 1.0;
    g_sphere.material.transparency = 0.9;
    g_sphere.material.refractive_index = 1.52;

    let mut a_sphere = Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5)));
    a_sphere.material.diffuse = 0.1;
    a_sphere.material.shininess = 300.;
    a_sphere.material.reflective = 1.;
    a_sphere.material.transparency = 0.9;
    a_sphere.material.refractive_index = 1.00029;
    a_sphere.material.pattern = Box::new(Solid::new(Color::new(0.0, 0.0, 0.0)));

    let mut world = World::new();
    world.objects = vec![
        Box::new(floor),
        Box::new(g_sphere),
        Box::new(a_sphere)
    ];
    world.light_sources = vec![PointLight::new(
        Color::new(1.0, 1.0, 1.0),
        Tuple::point(-10.0, 10.0, -10.),
    )];

    let camera = Camera::new_with_transform(
        512,
        512,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 2.5, 0.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
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
