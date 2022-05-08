use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{transform_ray_to_object_space, Intersectable, Intersection};

pub struct Plane {
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub material: Material,
}

impl Plane {
    pub fn new(transform: Option<Matrix>) -> Plane {
        match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                // cache some matrices so we don't need to calculate it every time
                let inverse = matrix.inverse();
                let mut inv_transpose = matrix.inverse();
                inv_transpose.transpose();
                Plane {
                    transform: matrix,
                    inverse_transform: inverse,
                    inverse_transform_transpose: inv_transpose,
                    material: Material::default_material(),
                }
            }
            None => Plane {
                transform: Matrix::identity(4),
                inverse_transform: Matrix::identity(4),
                inverse_transform_transpose: Matrix::identity(4),
                material: Material::default_material(),
            },
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

    fn normal_at(&self, _: Tuple) -> Tuple {
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
}

#[cfg(test)]
mod test {

    use crate::math::utils::f64_eq;

    use super::*;

    #[test]
    fn normal_of_place_is_constant() {
        let p = Plane::new(None);
        assert_eq!(
            p.normal_at(Tuple::point(0.0, 0.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.normal_at(Tuple::point(10.0, 0.0, -10.0)),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            p.normal_at(Tuple::point(-5.0, 0.0, 150.0)),
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
