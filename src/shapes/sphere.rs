use crate::math::{matrix::Matrix, ray::Ray, tuples::Tuple};

use super::intersect::{Intersect, Intersection};

pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    pub fn new(transform: Option<Matrix>) -> Sphere {
        match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                Sphere {
                    transform: matrix,
                }
            }
            None => Sphere {
                transform: Matrix::identity(4),
            },
        }
    }
}

impl Intersect for Sphere {
    /*
        Determine at what points the ray interests the sphere, if any
    */
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let inv = self.transform.inverse();
        let transformed_ray = ray.apply_transform(&inv);

        let sphere_to_ray = transformed_ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * sphere_to_ray.dot(&transformed_ray.direction);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // ray missed sphere
            return vec![];
        }

        vec![
            Intersection {
                shape: self,
                t: (-b - discriminant.sqrt()) / (2.0 * a),
            },
            Intersection {
                shape: self,
                t: (-b + discriminant.sqrt()) / (2.0 * a),
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
        let s = Sphere::new(None);
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0)
    }

    #[test]
    fn ray_intersect_sphere_top() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0)
    }

    #[test]
    fn ray_intersect_sphere_miss() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let xs = s.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_intersect_sphere_cast_from_origin() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_intersect_sphere_cas_from_behind_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(Matrix::scaling(2.0, 2.0, 2.0)));
        let xs = s.intersect(&ray);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(Matrix::translation(5.0, 0.0, 0.0)));
        let xs = s.intersect(&ray);
        assert!(xs.is_empty());
    }
}
