use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{
   Intersectable, Intersection, OBJECT_COUNTER,
};

pub struct Cone {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub parent: Option<usize>,
    pub material: Material,
    pub minimum: f64, // bottom cone cutoff
    pub maximum: f64, // top cone cutoff
    pub closed: bool, // wether not not to cap the cone
}

impl Cone {
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
        if check_cap(ray, t0, self.minimum) {
            xs.push(Intersection { shape: self, t: t0 });
        }

        // check for an intersection at the top cap
        let t1 = (self.maximum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t1, self.maximum) {
            xs.push(Intersection { shape: self, t: t1 });
        }

        xs
    }
}

fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    (x.powi(2) + z.powi(2)) <= radius.powi(2)
}

impl Intersectable for Cone {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.direction.x * ray.origin.x - 2.0 * ray.direction.y * ray.origin.y
            + 2.0 * ray.direction.z * ray.origin.z;
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        let mut intersects = vec![];

        if a.abs() <= EPSILON && b.abs() > EPSILON {
            intersects.push(Intersection {
                shape: self,
                t: -c / (2.0 * b),
            });
        }

        if a.abs() > EPSILON {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc >= 0.0 {
                let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                let y0 = ray.origin.y + t0 * ray.direction.y;
                if self.minimum < y0 && y0 < self.maximum {
                    intersects.push(Intersection { shape: self, t: t0 });
                }

                let y1 = ray.origin.y + t1 * ray.direction.y;
                if self.minimum < y1 && y1 < self.maximum {
                    intersects.push(Intersection { shape: self, t: t1 });
                }
            }
        }

        let mut cap_intersects = self.intersect_caps(ray);
        intersects.append(&mut cap_intersects);
        intersects
    }

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        let dist = object_point.x.powi(2) + object_point.z.powi(2);

        if dist < 1.0 && object_point.y >= self.maximum - EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && object_point.y <= self.minimum + EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            let mut y = dist.sqrt();
            if object_point.y > 0.0 {
                y *= -1.0;
            }
            Tuple::vector(object_point.x, y, object_point.z)
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

#[cfg(test)]
mod test {
    use crate::math::{ray::Ray, tuples::Tuple, utils::f64_eq};

    use super::*;

    #[test]
    fn intersecting_cone() {
        let origins = vec![
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(1.0, 1.0, -5.0),
        ];

        let direction = vec![
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(1.0, 1.0, 1.0),
            Tuple::vector(-0.5, -1.0, 1.0),
        ];

        let ans = vec![(5.0, 5.0), (8.66025, 8.66025), (4.55006, 49.449944)];

        let cone = Cone::new(None);

        for i in 0..origins.len() {
            let dir = direction[i].normalize();
            let ray = Ray::new(origins[i], dir);
            let xs = cone.local_intersect(&ray);
            assert!(f64_eq(xs[0].t, ans[i].0));
            assert!(f64_eq(xs[1].t, ans[i].1));
        }
    }

    #[test]
    fn intersection_with_parallel_ray_to_half() {
        let cone = Cone::new(None);
        let dir = Tuple::vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Tuple::point(0.0, 0.0, -1.0), dir);
        let xs = cone.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(f64_eq(xs[0].t, 0.35355));
    }

    #[test]
    fn intersecting_end_caps() {
        let origins = vec![
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -0.25),
            Tuple::point(0.0, 0.0, -0.25),
        ];

        let direction = vec![
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 1.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ];

        let ans = vec![0, 2, 4];

        let mut cone = Cone::new(None);
        cone.closed = true;
        cone.minimum = -0.5;
        cone.maximum = 0.5;

        for i in 0..origins.len() {
            let dir = direction[i].normalize();
            let ray = Ray::new(origins[i], dir);
            let xs = cone.local_intersect(&ray);
            assert_eq!(xs.len(), ans[i]);
        }
    }

    #[test]
    fn normal_works() {
        let points = vec![
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(1.0, 1.0, 1.0),
            Tuple::point(-1.0, -1.0, 0.0),
        ];

        let normals = vec![
            Tuple::vector(0.0, 0.0, 0.0),
            Tuple::vector(1.0, -(2.0_f64.sqrt()), 1.0),
            Tuple::vector(-1.0, 1.0, 0.0),
        ];

        let cone = Cone::new(None);

        for i in 0..points.len() {
            let n = cone.local_normal_at(points[i]);
            assert_eq!(n, normals[i]);
        }
    }
}
