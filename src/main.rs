#![allow(dead_code, non_snake_case)]

use draw::{canvas::Canvas, color::Color, light::PointLight};
use math::{ray::Ray, tuples::Tuple};
use shapes::{
    intersect::{hit, Intersect},
    sphere::Sphere,
};

mod draw;
mod math;
mod shapes;
fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 1000;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size as f32 / 2.0;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    let s = Sphere::new(None);

    let light = PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::point(world_x as f32, world_y as f32, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = s.intersect(&r);
            let hits = hit(xs);
            if let Some(h) = hits {
                let point = r.position(h.t);
                let normal = s.normal_at(point);
                let eye = -r.direction;
                c.write_pixel(x, y, light.lighting(s.material, point, eye, normal));
            }
        }
    }

    c.write_to_ppm();
}
