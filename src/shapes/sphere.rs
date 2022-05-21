use std::sync::atomic::Ordering;

use crate::{
    draw::{color::Color, material::Material, patterns::Solid},
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::intersect::{Intersectable, Intersection, OBJECT_COUNTER};

pub struct Sphere {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
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
            parent: None,
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
    fn local_normal_at(&self, object_point: Tuple, _: Intersection) -> Tuple {
        // find the normal vector in object space (i.e. a unit sphere at the origin)
        object_point - Tuple::point(0.0, 0.0, 0.0)
    }

    /*
        Determine at what points the ray intersects the sphere, if any
    */
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // cast the ray
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        // calculate the discriminant
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * sphere_to_ray.dot(&ray.direction);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // ray missed sphere
            return vec![];
        }

        vec![
            Intersection::new(self, (-b - discriminant.sqrt()) / (2.0 * a)),
            Intersection::new(self, (-b + discriminant.sqrt()) / (2.0 * a)),
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

    fn get_parent_id(&self) -> Option<usize> {
        self.parent
    }

    fn set_parent_id(&mut self, id: usize) {
        self.parent = Some(id);
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
        let dummy_hit = Intersection::new(&s, 0.0);
        let n = s.local_normal_at(Tuple::point(1.0, 0.0, 0.0), dummy_hit);
        assert!(n == Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_sphere_y_axis() {
        let s = Sphere::new(None);
        let dummy_hit = Intersection::new(&s, 0.0);
        let n = s.local_normal_at(Tuple::point(0.0, 1.0, 0.0), dummy_hit);
        assert!(n == Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_sphere_z_axis() {
        let s = Sphere::new(None);
        let dummy_hit = Intersection::new(&s, 0.0);
        let n = s.local_normal_at(Tuple::point(0.0, 0.0, 1.0), dummy_hit);
        assert!(n == Tuple::vector(0.0, 0.0, 1.0));
    }
}
