use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::intersect::{Intersect, Intersection};

pub struct Sphere {
    transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: Option<Matrix>) -> Sphere {
        match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                Sphere {
                    transform: matrix,
                    material: Material::default_material(),
                }
            }
            None => Sphere {
                transform: Matrix::identity(4),
                material: Material::default_material(),
            },
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let mut inv_sphere_transform = self.transform.inverse();
        // convert form world space to object space
        let object_point = &inv_sphere_transform * &world_point;
        // find the normal vector in object space
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);

        // convert the normal vector in object space back to world space
        inv_sphere_transform.transpose();
        let mut world_normal = &inv_sphere_transform * &object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
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
    use std::f32::consts::PI;

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

    #[test]
    fn normal_at_sphere_x_axis() {
        let s = Sphere::new(None);
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert!(n == Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_sphere_y_axis() {
        let s = Sphere::new(None);
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert!(n == Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_sphere_z_axis() {
        let s = Sphere::new(None);
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert!(n == Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_vector_normalized() {
        let s = Sphere::new(None);
        let n = s.normal_at(Tuple::point(0.5, 1.0, 0.33));
        assert!(n == n.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Sphere::new(Some(Matrix::translation(0.0, 1.0, 0.0)));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert!(n == Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Sphere::new(Some(
            &Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(PI / 5.0),
        ));

        let n = s.normal_at(Tuple::point(
            0.0,
            (2.0_f32).sqrt() / 2.0,
            -(2.0_f32).sqrt() / 2.0,
        ));
        assert!(n == Tuple::vector(0.0, 0.97014, -0.24254));
    }
}
