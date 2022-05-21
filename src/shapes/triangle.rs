use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::{
    intersect::{Intersectable, Intersection, OBJECT_COUNTER},
    ttriangle::{moller_trumbore_inner, TTriangle},
};

pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
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

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, transform: Option<Matrix>) -> Self {
        assert!(p1.is_point());
        assert!(p2.is_point());
        assert!(p3.is_point());
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

impl TTriangle for Triangle {
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

impl Intersectable for Triangle {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match moller_trumbore_inner(self, ray) {
            Some(values) => vec![Intersection::new(self, values.0)],
            None => vec![],
        }
    }

    fn local_normal_at(&self, _: Tuple, _: Intersection) -> Tuple {
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

    fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        math::{ray::Ray, tuples::Tuple},
        shapes::intersect::{Intersectable, Intersection},
    };

    use super::Triangle;

    #[test]
    fn triangle_initialized_correctly() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let dummy_hit = Intersection::new(&t, 0.0);
        assert_eq!(t.e1, Tuple::vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Tuple::vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(
            t.normal,
            t.local_normal_at(Tuple::vector(0.0, 0.0, 0.0), dummy_hit)
        );
    }

    #[test]
    fn ray_misses_triangle() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let r = Ray::new(Tuple::point(1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let r = Ray::new(Tuple::point(-1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersects_triangle() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            None,
        );
        let r = Ray::new(Tuple::point(0.0, 0.5, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}
