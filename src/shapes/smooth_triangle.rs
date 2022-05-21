use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
    shapes::intersect::OBJECT_COUNTER,
};

use super::{
    intersect::{Intersectable, Intersection},
    ttriangle::{moller_trumbore_inner, TTriangle},
};

pub struct SmoothTriangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
    pub material: Material,
    e1: Tuple,
    e2: Tuple,
    normal: Tuple,
}

impl SmoothTriangle {
    pub fn new(
        p1: Tuple,
        p2: Tuple,
        p3: Tuple,
        n1: Tuple,
        n2: Tuple,
        n3: Tuple,
        transform: Option<Matrix>,
    ) -> Self {
        assert!(p1.is_point());
        assert!(p2.is_point());
        assert!(p3.is_point());
        assert!(n1.is_vector());
        assert!(n2.is_vector());
        assert!(n3.is_vector());

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

        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(&e1).normalize();
        assert!(e1.is_vector());
        assert!(e2.is_vector());
        assert!(normal.is_vector());

        Self {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            transform: matrices.0,
            inverse_transform: matrices.1,
            inverse_transform_transpose: matrices.2,
            material: Material::default_material(),
            id,
            parent: None,
            e1,
            e2,
            normal,
        }
    }
}

impl TTriangle for SmoothTriangle {
    fn e1(&self) -> Tuple {
        self.e1
    }

    fn e2(&self) -> Tuple {
        self.e2
    }

    fn p1(&self) -> Tuple {
        self.p1
    }
}

impl Intersectable for SmoothTriangle {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match moller_trumbore_inner(self, ray) {
            Some(values) => vec![Intersection::new_uv(self, values.0, values.1, values.2)],
            None => vec![],
        }
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        self.normal
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
    use crate::{math::{tuples::Tuple, ray::Ray, utils::f64_eq}, shapes::intersect::Intersectable};

    use super::SmoothTriangle;

    fn test_triangle() -> SmoothTriangle {
        SmoothTriangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
            None,
        )
    }

    #[test]
    fn u_v_calculated_correctly() {
        let tri = test_triangle();
        let ray = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = tri.local_intersect(&ray);
        assert!(f64_eq(xs[0].u.unwrap(), 0.45));
        assert!(f64_eq(xs[0].v.unwrap(), 0.25));
    }
}
