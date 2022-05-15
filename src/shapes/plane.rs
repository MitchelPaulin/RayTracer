use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{
    transform_ray_to_object_space, Intersectable, Intersection,
    OBJECT_COUNTER,
};

pub struct Plane {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
    pub material: Material,
}

impl Plane {
    pub fn new(transform: Option<Matrix>) -> Plane {
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
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = transform_ray_to_object_space(self, ray);

        // for the purposes of keeping the calculations easy assume the plane is flat in the xz direction

        // the ray is parallel to the plane, thus it will never intersect it
        if transformed_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        vec![Intersection {
            shape: self,
            t: -transformed_ray.origin.y / transformed_ray.direction.y,
        }]
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
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
        self.parent = Some(id)
    }
}

#[cfg(test)]
mod test {

    use crate::math::utils::f64_eq;

    use super::*;

    #[test]
    fn normal_of_place_is_constant() {
        let p = Plane::new(None);
        assert_eq!(
            p.local_normal_at(Tuple::point(0.0, 0.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.local_normal_at(Tuple::point(10.0, 0.0, -10.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn ray_is_parallel_to_plane() {
        let p = Plane::new(None);
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_co_planner() {
        let p = Plane::new(None);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_above() {
        let p = Plane::new(None);
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(f64_eq(xs[0].t, 1.0));
    }

    #[test]
    fn ray_intersecting_plane_below() {
        let p = Plane::new(None);
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(f64_eq(xs[0].t, 1.0));
    }
}
