use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::intersect::{Intersectable, Intersection, OBJECT_COUNTER};

pub struct Group {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
    pub material: Material,
    pub objects: Vec<Box<dyn Intersectable>>,
}

impl Group {
    pub fn new(transform: Option<Matrix>) -> Self {
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
            objects: vec![],
            parent: None,
        }
    }

    pub fn add_object(&mut self, mut shape: Box<dyn Intersectable>) {
        shape.set_parent_id(self.id);
        self.objects.push(shape);
    }

    pub fn get_object(&self, index: usize) -> Option<&dyn Intersectable> {
        match self.objects.get(index) {
            Some(o) => Some(o.as_ref()),
            None => None,
        }
    }
}

impl Intersectable for Group {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersects = vec![];
        for s in &self.objects {
            intersects.append(&mut s.intersect(ray));
        }
        intersects.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersects
    }

    fn get_object_by_id(&self, id: usize) -> Option<&dyn Intersectable> {
        let mut shape = None;
        for s in &self.objects {
            if s.get_id() == id {
                shape = Some(s.as_ref());
                break;
            }
            if let Some(c) = s.get_object_by_id(id) {
                shape = Some(c);
                break;
            }
        }

        shape
    }

    fn local_normal_at(&self, _: Tuple, _: Intersection) -> Tuple {
        panic!("A group does not have a normal, something went wrong")
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

    use std::f64::consts::PI;

    use crate::{scene::world::World, shapes::sphere::Sphere};

    use super::*;

    #[test]
    fn shape_added_successfully() {
        let mut g = Group::new(None);
        let s = Sphere::new(None);
        let id = s.get_id();
        g.add_object(Box::new(s));
        assert!(!g.objects.is_empty());
        assert_eq!(g.objects[0].get_id(), id);
        assert_eq!(g.objects[0].get_parent_id().unwrap(), g.get_id());
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let g = Group::new(None);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_with_non_empty_group() {
        let mut g = Group::new(None);
        let s1 = Sphere::new(None);
        let s2 = Sphere::new(Some(Matrix::translation(0.0, 0.0, -3.0)));
        let s3 = Sphere::new(Some(Matrix::translation(5.0, 0.0, 0.0)));
        let s1_id = s1.get_id();
        let s2_id = s2.get_id();
        g.add_object(Box::new(s1));
        g.add_object(Box::new(s2));
        g.add_object(Box::new(s3));

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].shape.get_id(), s2_id);
        assert_eq!(xs[1].shape.get_id(), s2_id);
        assert_eq!(xs[2].shape.get_id(), s1_id);
        assert_eq!(xs[3].shape.get_id(), s1_id);
    }

    #[test]
    fn intersecting_transformed_groups() {
        let mut g = Group::new(Some(Matrix::scaling(2.0, 2.0, 2.0)));
        let s = Sphere::new(Some(Matrix::translation(5.0, 0.0, 0.0)));
        g.add_object(Box::new(s));
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut g1 = Group::new(Some(Matrix::rotation_y(PI / 2.0)));
        let mut g2 = Group::new(Some(Matrix::scaling(1.0, 2.0, 3.0)));
        let g2_id = g2.get_id();
        let s = Sphere::new(Some(Matrix::translation(5.0, 0.0, 0.0)));
        let s_id = s.get_id();
        g2.add_object(Box::new(s));
        g1.add_object(Box::new(g2));

        let mut dummy_world = World::new();
        dummy_world.objects = vec![Box::new(g1)];

        let s = Sphere::new(None);
        let dummy_hit = Intersection::new(&s, 0.0);

        let n = dummy_world.objects[0]
            .get_object_by_id(g2_id)
            .unwrap()
            .get_object_by_id(s_id)
            .unwrap()
            .normal_at(
                Tuple::point(1.7321, 1.1547, -5.5774),
                dummy_hit,
                Some(&dummy_world),
            );
        assert_eq!(
            n,
            Tuple::vector(
                0.28570368184140726,
                0.42854315178114105,
                -0.8571605294481017
            )
        )
    }
}
