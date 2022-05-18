#![allow(dead_code, non_snake_case)]

use std::{f64::consts::PI, sync::atomic::Ordering, time::Instant};

use draw::color::Color;
use math::{matrix::Matrix, tuples::Tuple};
use scene::{
    camera::{render, view_transform, Camera},
    light::PointLight,
    world::World,
};
use shapes::group::Group;

use crate::{
    draw::patterns::{Checkered, Rings, Solid},
    scene::world::RAY_INTERSECT_COUNTER,
    shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
};

mod draw;
mod math;
mod scene;
mod shapes;
fn main() {
    let scene = test_scene();

    let start = Instant::now();
    let image = render(scene.0, scene.1, 6);
    image.write_to_ppm("canvas.ppm");
    let intersects = RAY_INTERSECT_COUNTER.load(Ordering::SeqCst);
    println!(
        "Image rendering finished in {:?} with {} ray-object intersections at a speed of {} intersections/ms.\nFile written to canvas.ppm",
        Instant::now().duration_since(start),
        intersects,
        (intersects as f64 / Instant::now().duration_since(start).as_millis() as f64).round()
    );
}

fn test_scene() -> (Camera, World) {
    let mut g = Group::new(None);

    let mut middle = Sphere::new(Some(Matrix::translation(-0.5, 1.0, 0.5)));
    middle.material.pattern = Box::new(Solid::new(Color::black()));
    middle.material.specular = 1.;
    middle.material.transparency = 1.0;
    middle.material.reflective = 0.9;
    middle.material.shininess = 300.;
    middle.material.ambient = 0.1;
    middle.material.diffuse = 0.1;
    middle.material.refractive_index = 1.52;

    let mut middle_behind = Cube::new(Some(
        &Matrix::translation(0.5, 1.0, 4.) * &Matrix::rotation_y(PI / 3.),
    ));
    middle_behind.material.pattern = Box::new(Solid::new(Color::new(1.0, 0.0, 0.0)));
    middle_behind.material.diffuse = 0.7;
    middle_behind.material.specular = 0.3;
    middle_behind.material.shininess = 100.;
    middle_behind.material.reflective = 0.1;

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

    let mut cylinder_outer = Cylinder::new(Some(Matrix::translation(-2.5, 0.0, 4.0)));
    cylinder_outer.minimum = 0.0;
    cylinder_outer.maximum = 1.0;
    cylinder_outer.closed = true;
    cylinder_outer.material.pattern = Box::new(Solid::new(Color::new(1.0, 0.3, 1.0)));
    cylinder_outer.material.specular = 1.;
    cylinder_outer.material.shininess = 20.;
    cylinder_outer.material.ambient = 0.5;
    cylinder_outer.material.diffuse = 0.1;
    cylinder_outer.material.reflective = 0.2;

    let mut cylinder_middle = Cylinder::new(Some(
        &(&Matrix::rotation_x(PI / -2.) * &(Matrix::scaling(0.66, 1.0, 0.66)))
            * &Matrix::translation(-4.0, -5.0, 2.5),
    ));
    cylinder_middle.minimum = 1.0;
    cylinder_middle.maximum = 1.5;
    cylinder_middle.closed = true;
    cylinder_middle.material.pattern = Box::new(Solid::new(Color::new(0.0, 1.0, 0.0)));
    cylinder_middle.material.refractive_index = 1.52;
    cylinder_middle.material.transparency = 0.7;
    cylinder_middle.material.specular = 1.;
    cylinder_middle.material.reflective = 0.9;
    cylinder_middle.material.shininess = 150.;
    cylinder_middle.material.ambient = 0.1;
    cylinder_middle.material.diffuse = 0.1;

    let mut cone = Cone::new(Some(
        &(&(&(&Matrix::rotation_x(PI / 2.) * &Matrix::rotation_z(PI / -3.))
            * &Matrix::rotation_x(PI / -7.4))
            * &Matrix::scaling(1.0, 2.0, 1.0))
            * &Matrix::translation(-1.0, 1.0, 1.0),
    ));
    cone.minimum = 0.0;
    cone.maximum = 1.0;
    cone.closed = true;
    cone.material.pattern = Box::new(Solid::new(Color::new(1.0, 1.0, 0.0)));
    cone.material.refractive_index = 1.52;
    cone.material.transparency = 0.7;
    cone.material.specular = 1.;
    cone.material.reflective = 0.9;
    cone.material.shininess = 150.;
    cone.material.ambient = 0.5;
    cone.material.diffuse = 0.1;
    cone.material.ambient = 0.2;

    let mut world = World::new();
    g.add_object(Box::new(left));
    g.add_object(Box::new(middle));
    g.add_object(Box::new(right));
    g.add_object(Box::new(floor));
    g.add_object(Box::new(middle_behind));
    g.add_object(Box::new(cylinder_outer));
    g.add_object(Box::new(cylinder_middle));
    g.add_object(Box::new(cone));
    g.add_object(Box::new(ceil));

    world.objects = vec![Box::new(g)];
    world.light_sources = vec![PointLight::new(
        Color::new(1.0, 1.0, 1.0),
        Tuple::point(-10.0, 13.0, -10.),
    )];

    let camera = Camera::new_with_transform(
        1920,
        1080,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 3.0, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    (camera, world)
}
