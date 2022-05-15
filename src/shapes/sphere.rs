use std::sync::atomic::Ordering;

use crate::{
    draw::{color::Color, material::Material, patterns::Solid},
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::intersect::{
    object_space_to_world_space, transform_ray_to_object_space, Intersectable, Intersection,
    OBJECT_COUNTER,
};

pub struct Sphere {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: Option<Matrix>) -> Sphere {
        let id = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let matrices = match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                let inverse = matrix.inverse();
                let mut inv_transpose = matrix.inverse();
                inv_transpose.transpose();
                (matrix, inverse, inv_transpose)
            }
            None => (
                Matrix::identity(4),
                Matrix::identity(4),
                Matrix::identity(4),
            ),
        };

        Self {
            transform: matrices.0,
            inverse_transform: matrices.1,
            inverse_transform_transpose: matrices.2,
            material: Material::default_material(),
            id,
        }
    }

    pub fn new_glass_sphere(transform: Option<Matrix>) -> Sphere {
        let mut gs = Sphere::new(transform);
        gs.material.transparency = 1.0;
        gs.material.refractive_index = 1.5;
        gs.material.pattern = Box::new(Solid::new(Color::black()));
        gs
    }
}

impl Intersectable for Sphere {
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        // convert form world space to object space
        let object_point = self.get_inverse_transform() * &world_point;

        // find the normal vector in object space (i.e. a unit sphere at the origin)
        // different for every shape
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);

        // convert the normal vector in object space back to world space
        object_space_to_world_space(self, &object_normal)
    }

    /*
        Determine at what points the ray intersects the sphere, if any
    */
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = transform_ray_to_object_space(self, ray);

        // cast the ray
        let sphere_to_ray = transformed_ray.origin - Tuple::point(0.0, 0.0, 0.0);

        // calculate the discriminant
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

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn get_inverse_transform_transpose(&self) -> &Matrix {
        &self.inverse_transform_transpose
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

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
        assert!(xs.is_empty());
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
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert!(n == Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Sphere::new(Some(
            &Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(PI / 5.0),
        ));

        let n = s.normal_at(Tuple::point(
            0.0,
            (2.0_f64).sqrt() / 2.0,
            -(2.0_f64).sqrt() / 2.0,
        ));
        assert!(n == Tuple::vector(0.0, 0.97014, -0.24254));
    }
}
