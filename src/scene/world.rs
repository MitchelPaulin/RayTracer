use crate::{
    draw::{color::Color, light::PointLight},
    math::{ray::Ray, tuples::Tuple},
    shapes::intersect::{hit, prepare_computations, Computations, Intersectable, Intersection},
};

pub struct World {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub light_sources: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            light_sources: vec![],
        }
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        for s in &self.objects {
            intersections.append(&mut s.intersect(ray));
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    pub fn shade_hit(&self, object: &dyn Intersectable, comps: &Computations) -> Color {
        let mut color = Color::black();

        for light in &self.light_sources {
            color += light.lighting(
                object,
                comps.object.get_material(),
                comps.over_point,
                comps.eyev,
                comps.normalv,
                self.is_shadowed(light, &comps.over_point),
            );
        }

        color
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect_world(ray);
        match hit(intersections) {
            Some(hit) => {
                let comps = prepare_computations(&hit, ray);
                self.shade_hit(hit.shape, &comps)
            }
            None => Color::black(),
        }
    }

    fn is_shadowed(&self, light_source: &PointLight, point: &Tuple) -> bool {
        assert!(point.is_point());

        // get the vector from the point to the light source
        let v = light_source.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();

        // cast a ray from that point towards the source of light
        let r = Ray::new(*point, direction);
        let intersections = self.intersect_world(&r);
        let h = hit(intersections);

        // if this ray collided with an object on it way to the light, return true otherwise false
        match h {
            Some(hit) => hit.t < distance,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{
        draw::{color::Color, material::Material, patterns::Solid},
        math::{matrix::Matrix, tuples::Tuple, utils::f64_eq},
        scene::camera::{render, view_transform, Camera},
        shapes::{intersect::prepare_computations, sphere::Sphere},
    };

    use super::*;

    fn get_populated_world() -> World {
        let mut w = World::new();

        w.light_sources.push(PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Tuple::point(-10.0, 10.0, -10.0),
        ));

        let mut s1 = Sphere::new(None);
        s1.material = Material::default_material();
        s1.material.pattern = Box::new(Solid::new(Color::new(0.8, 1.0, 0.6)));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5)));
        s2.material = Material::default_material();
        s2.material.pattern = Box::new(Solid::new(Color::new(0.8, 1.0, 0.6)));
        s2.material.diffuse = 0.7;
        s2.material.specular = 0.2;

        w.objects.push(Box::new(s1));
        w.objects.push(Box::new(s2));

        w
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = get_populated_world();
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let transform = view_transform(from, to, up);
        let c = Camera::new_with_transform(11, 11, PI / 2.0, transform);
        let image = render(c, w, 1);
        assert_eq!(image.get_pixel(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn default_world_intersection() {
        let world = get_populated_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = world.intersect_world(&ray);
        assert_eq!(intersections.len(), 4);
        assert!(f64_eq(intersections[0].t, 4.0));
        assert!(f64_eq(intersections[1].t, 4.5));
        assert!(f64_eq(intersections[2].t, 5.5));
        assert!(f64_eq(intersections[3].t, 6.0));
    }

    #[test]
    fn shading_an_intersection() {
        let dummy_sphere = Sphere::new(None);
        let w = get_populated_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = w.intersect_world(&ray);
        let comps = prepare_computations(&intersections[0], &ray);
        let c = w.shade_hit(&dummy_sphere, &comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn ray_miss() {
        let w = get_populated_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(w.color_at(&ray), Color::black());
    }

    #[test]
    fn ray_hit() {
        let w = get_populated_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        assert_eq!(w.color_at(&ray), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn intersection_behind_ray() {
        let mut w = get_populated_world();

        let mut s1 = Sphere::new(None);
        s1.material.ambient = 1.0;
        s1.material.pattern = Box::new(Solid::new(Color::new(0.1, 0.2, 0.3)));
        w.objects[0] = Box::new(s1);

        let mut s2 = Sphere::new(None);
        s2.material.ambient = 1.0;
        w.objects[1] = Box::new(s2);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(w.color_at(&ray), Color::new(0.1, 0.2, 0.3));
    }

    #[test]
    fn no_shadow() {
        let w = get_populated_world();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&w.light_sources[0], &p));
    }

    #[test]
    fn is_shadow_behind_object() {
        let w = get_populated_world();
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&w.light_sources[0], &p));
    }

    #[test]
    fn no_shadow_point_behind_light() {
        let w = get_populated_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&w.light_sources[0], &p));
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let w = get_populated_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&w.light_sources[0], &p));
    }
}
