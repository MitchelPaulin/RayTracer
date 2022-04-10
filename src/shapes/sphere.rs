use crate::math::{ray::Ray, tuples::Tuple};

use super::intersect::{Intersect, Intersection};

pub struct Sphere {
    origin: Tuple,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}

impl Intersect for Sphere {
    /*
        Determine at what points the ray interests the sphere, if any
    */
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // ray missed sphere
            return vec![];
        }

        vec![
            Intersection {
                shape: self,
                intersection: (-b - discriminant.sqrt()) / 2.0 * a,
            },
            Intersection {
                shape: self,
                intersection: (-b + discriminant.sqrt()) / 2.0 * a,
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::math::tuples::Tuple;

    use super::*;

    #[test]
    fn ray_intersect_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs[0].intersection, 4.0);
        assert_eq!(xs[1].intersection, 6.0)
    }

    #[test]
    fn ray_intersect_sphere_top() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs[0].intersection, 5.0);
        assert_eq!(xs[1].intersection, 5.0)
    }

    #[test]
    fn ray_intersect_sphere_miss() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_intersect_sphere_cast_from_origin() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs[0].intersection, -1.0);
        assert_eq!(xs[1].intersection, 1.0);
    }

    #[test]
    fn ray_intersect_sphere_cas_from_behind_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs[0].intersection, -6.0);
        assert_eq!(xs[1].intersection, -4.0);
    }
}
