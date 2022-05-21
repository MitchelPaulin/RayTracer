use std::{f64::consts::PI, fs};

use crate::{
    draw::{
        color::Color,
        material::Material,
        patterns::{Checkered, Rings, Solid},
    },
    math::{matrix::Matrix, tuples::Tuple},
    obj_parser::parse_obj_file,
    scene::{
        camera::{view_transform, Camera},
        light::PointLight,
        world::World,
    },
    shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
};

pub fn pawn_chess() -> (Camera, World) {
    let mut world = World::new();

    let obj =
        fs::read_to_string("./obj/pawn-chess.obj").expect("Something went wrong reading the file");

    let mut pawn_mat = Material::default_material();
    pawn_mat.specular = 1.;
    pawn_mat.transparency = 1.0;
    pawn_mat.reflective = 0.9;
    pawn_mat.shininess = 300.;
    pawn_mat.ambient = 0.1;
    pawn_mat.diffuse = 0.1;
    pawn_mat.refractive_index = 1.52;

    
    let g = parse_obj_file(&obj, Some(pawn_mat));

    let mut plane = Plane::new(Some(Matrix::scaling(2.0, 2.0, 2.0)));
    plane.material.pattern = Box::new(Checkered::new(Color::black(), Color::white()));
    plane.material.reflective = 0.3;

    world.objects = vec![Box::new(g), Box::new(plane)];

    world.light_sources = vec![PointLight::new(
        Color::new(1.0, 1.0, 1.0),
        Tuple::point(-10.0, 13.0, -10.),
    )];

    let camera = Camera::new_with_transform(
        1000,
        1000,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 4.0, -5.0),
            Tuple::point(0.0, 2.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    (camera, world)
}

pub fn book_cover() -> (Camera, World) {
    let mut world = World::new();

    world.light_sources = vec![
        PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(50.0, 100.0, -50.0)),
        PointLight::new(Color::new(0.2, 0.2, 0.2), Tuple::point(-400.0, 50.0, -10.0)),
    ];

    let mut white_material = Material::default_material();
    white_material.pattern = Box::new(Solid::new(Color::white()));
    white_material.diffuse = 0.7;
    white_material.ambient = 0.1;
    white_material.specular = 0.0;
    white_material.reflective = 0.1;

    let mut blue_material = Material::from_material(&white_material);
    blue_material.pattern = Box::new(Solid::new(Color::new(0.537, 0.831, 0.914)));

    let mut red_material = Material::from_material(&white_material);
    red_material.pattern = Box::new(Solid::new(Color::new(0.941, 0.322, 0.388)));

    let mut purple_material = Material::from_material(&white_material);
    purple_material.pattern = Box::new(Solid::new(Color::new(0.373, 0.404, 0.550)));

    let standard_transform = &Matrix::scaling(0.5, 0.5, 0.5) * &Matrix::translation(1.0, -1.0, 1.0);

    let large_object = &Matrix::scaling(3.5, 3.5, 3.5) * &standard_transform;

    let medium_object = &Matrix::scaling(3.0, 3.0, 3.0) * &standard_transform;

    let small_object = &Matrix::scaling(2.0, 2.0, 2.0) * &standard_transform;

    let mut plane = Plane::new(Some(
        &Matrix::translation(0.0, 0.0, 500.0) * &Matrix::rotation_x(PI / 2.0),
    ));
    plane.material.pattern = Box::new(Solid::new(Color::white()));
    plane.material.ambient = 1.0;
    plane.material.diffuse = 0.0;
    plane.material.specular = 0.0;
    world.objects.push(Box::new(plane));

    let mut sphere = Sphere::new(Some(&Matrix::identity(4) * &large_object));
    sphere.material.pattern = Box::new(Solid::new(Color::new(0.373, 0.404, 0.550)));
    sphere.material.diffuse = 0.2;
    sphere.material.ambient = 0.0;
    sphere.material.specular = 1.0;
    sphere.material.shininess = 200.0;
    sphere.material.reflective = 0.7;
    sphere.material.transparency = 0.7;
    sphere.material.refractive_index = 1.5;
    world.objects.push(Box::new(sphere));

    let mut cube = Cube::new(Some(&Matrix::translation(4.0, 0.0, 0.0) * &medium_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(8.5, 1.5, -0.5) * &large_object));
    cube.material = Material::from_material(&blue_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(0.0, 0.0, 4.0) * &large_object));
    cube.material = Material::from_material(&red_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(4.0, 0.0, 4.0) * &small_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(7.5, 0.5, 4.0) * &medium_object));
    cube.material = Material::from_material(&purple_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(
        &Matrix::translation(-0.25, 0.25, 8.0) * &medium_object,
    ));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(4.0, 1.0, 7.5) * &large_object));
    cube.material = Material::from_material(&blue_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(10.0, 2.0, 7.5) * &medium_object));
    cube.material = Material::from_material(&red_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(8.0, 2.0, 12.0) * &small_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(20.0, 1.0, 9.0) * &small_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(-0.5, -5.0, 0.25) * &large_object));
    cube.material = Material::from_material(&blue_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(4.0, -4.0, 0.0) * &large_object));
    cube.material = Material::from_material(&red_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(8.5, -4.0, 0.0) * &large_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(0.0, -4.0, 4.0) * &large_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(-0.5, -4.5, 8.0) * &large_object));
    cube.material = Material::from_material(&purple_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(0.0, -8.0, 4.0) * &large_object));
    cube.material = Material::from_material(&purple_material);
    world.objects.push(Box::new(cube));

    let mut cube = Cube::new(Some(&Matrix::translation(-0.5, -8.5, 8.0) * &large_object));
    cube.material = Material::from_material(&white_material);
    world.objects.push(Box::new(cube));

    let camera = Camera::new_with_transform(
        2000,
        2000,
        0.785,
        view_transform(
            Tuple::point(-6.0, 6.0, -10.0),
            Tuple::point(6.0, 0.0, 6.0),
            Tuple::vector(-0.45, 1.0, 0.0),
        ),
    );

    (camera, world)
}

pub fn test_scene() -> (Camera, World) {
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
    world.objects = vec![
        Box::new(left),
        Box::new(middle),
        Box::new(right),
        Box::new(floor),
        Box::new(middle_behind),
        Box::new(cylinder_outer),
        Box::new(cylinder_middle),
        Box::new(cone),
        Box::new(ceil),
    ];

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
