use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{Intersectable, Intersection, OBJECT_COUNTER};

pub struct Cylinder {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
    pub material: Material,
    pub minimum: f64, // bottom cylinder cutoff
    pub maximum: f64, // top cylinder cutoff
    pub closed: bool, // wether not not to cap the cylinder
}

impl Cylinder {
    pub fn new(transform: Option<Matrix>) -> Cylinder {
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
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
            parent: None,
        }
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<Intersection> {
        // if there are not caps to intersect or the ray is vertical, we have nothing to do
        if !self.closed || ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let mut xs = vec![];

        // check for an intersection at the bottom cap
        let t0 = (self.minimum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t0) {
            xs.push(Intersection::new(self, t0));
        }

        // check for an intersection at the top cap
        let t1 = (self.maximum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t1) {
            xs.push(Intersection::new(self, t1));
        }

        xs
    }
}

impl Intersectable for Cylinder {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        // ray is parallel to the cylinder, could still intersect a cap however
        if a.abs() < EPSILON {
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;

        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        // ray does not intersect cylinder
        if disc < 0.0 {
            return vec![];
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        let mut surface_intersects = vec![];

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            surface_intersects.push(Intersection::new(self, t0));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            surface_intersects.push(Intersection::new(self, t1));
        }

        let mut cap_intersects = self.intersect_caps(ray);
        surface_intersects.append(&mut cap_intersects);
        surface_intersects
    }

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        let dist = object_point.x.powi(2) + object_point.z.powi(2);

        if dist < 1.0 && object_point.y >= self.maximum - EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && object_point.y <= self.minimum + EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            Tuple::vector(object_point.x, 0.0, object_point.z)
        }
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

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    (x.powi(2) + z.powi(2)) <= 1.0
}

#[cfg(test)]
mod test {
    use crate::{
        math::{ray::Ray, tuples::Tuple, utils::f64_eq},
        shapes::intersect::Intersectable,
    };

    use super::Cylinder;

    #[test]
    fn ray_misses_cylinder() {
        let cyl = Cylinder::new(None);

        let origin = vec![
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, -5.0),
        ];

        let direction = vec![
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(1.0, 1.0, 1.0),
        ];

        for i in 0..origin.len() {
            let dir = direction[i].normalize();
            let ray = Ray::new(origin[i], dir);
            let xs = cyl.intersect(&ray);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn ray_intersects_cylinder() {
        let origin = vec![
            Tuple::point(1.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.5, 0.0, -5.0),
        ];

        let direction = vec![
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.1, 1.0, 1.0),
        ];

        let ts = vec![(5.0, 5.0), (4.0, 6.0), (6.80798, 7.08872)];

        let cyl = Cylinder::new(None);

        for i in 0..origin.len() {
            let dir = direction[i].normalize();
            let ray = Ray::new(origin[i], dir);
            let xs = cyl.intersect(&ray);
            assert_eq!(xs.len(), 2);
            assert!(f64_eq(xs[0].t, ts[i].0));
            assert!(f64_eq(xs[1].t, ts[i].1));
        }
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let points = vec![
            Tuple::point(0.0, 1.5, 0.0),
            Tuple::point(0.0, 3.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 2.0, -5.0),
            Tuple::point(0.0, 1.0, -5.0),
        ];

        let directions = vec![
            Tuple::vector(0.1, 1.0, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
        ];

        let mut cyl = Cylinder::new(None);
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;

        for i in 0..directions.len() {
            let dir = directions[i].normalize();
            let r = Ray::new(points[i], dir);
            let xs = cyl.intersect(&r);
            assert!(xs.is_empty());
        }

        let point = Tuple::point(0.0, 1.5, -2.0);
        let dir = Tuple::vector(0.0, 0.0, 1.0).normalize();
        let xs = cyl.intersect(&Ray::new(point, dir));
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let points = vec![
            Tuple::point(0.0, 3.0, 0.0),
            Tuple::point(0.0, 3.0, -2.0),
            Tuple::point(0.0, 4.0, -2.0),
            Tuple::point(0.0, 0.0, -2.0),
            Tuple::point(0.0, -1.0, -2.0),
        ];

        let directions = vec![
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, -1.0, 2.0),
            Tuple::vector(0.0, -1.0, 1.0),
            Tuple::vector(0.0, 1.0, 2.0),
            Tuple::vector(0.0, 1.0, 1.0),
        ];

        let mut cyl = Cylinder::new(None);
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;
        cyl.closed = true;

        for i in 0..directions.len() {
            let dir = directions[i].normalize();
            let r = Ray::new(points[i], dir);
            let xs = cyl.intersect(&r);
            assert_eq!(xs.len(), 2);
        }
    }
}
