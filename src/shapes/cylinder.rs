use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{
    transform_ray_to_object_space, Intersectable, Intersection, OBJECT_COUNTER, object_space_to_world_space,
};

pub struct Cylinder {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub material: Material,
    pub minimum: f64, // bottom cylinder cutoff
    pub maximum: f64, // top cylinder cutoff
    pub closed: bool, // wether not not to cap the cylinder
}

impl Cylinder {
    pub fn new(transform: Option<Matrix>) -> Cylinder {
        let id = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);
        match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                // cache some matrices so we don't need to calculate it every time
                let inverse = matrix.inverse();
                let mut inv_transpose = matrix.inverse();
                inv_transpose.transpose();
                Cylinder {
                    transform: matrix,
                    inverse_transform: inverse,
                    inverse_transform_transpose: inv_transpose,
                    material: Material::default_material(),
                    id,
                    minimum: f64::NEG_INFINITY,
                    maximum: f64::INFINITY,
                    closed: false,
                }
            }
            None => Cylinder {
                transform: Matrix::identity(4),
                inverse_transform: Matrix::identity(4),
                inverse_transform_transpose: Matrix::identity(4),
                material: Material::default_material(),
                id,
                minimum: f64::NEG_INFINITY,
                maximum: f64::INFINITY,
                closed: false,
            },
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
            xs.push(Intersection { shape: self, t: t0 });
        }

        // check for an intersection at the top cap
        let t1 = (self.maximum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t1) {
            xs.push(Intersection { shape: self, t: t1 });
        }

        xs
    }
}

impl Intersectable for Cylinder {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = transform_ray_to_object_space(self, ray);

        let a = transformed_ray.direction.x.powi(2) + transformed_ray.direction.z.powi(2);

        // ray is parallel to the cylinder, could still intersect a cap however
        if a.abs() < EPSILON {
            return self.intersect_caps(&transformed_ray);
        }

        let b = 2.0 * transformed_ray.origin.x * transformed_ray.direction.x
            + 2.0 * transformed_ray.origin.z * transformed_ray.direction.z;

        let c = transformed_ray.origin.x.powi(2) + transformed_ray.origin.z.powi(2) - 1.0;

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

        let y0 = transformed_ray.origin.y + t0 * transformed_ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            surface_intersects.push(Intersection { shape: self, t: t0 });
        }

        let y1 = transformed_ray.origin.y + t1 * transformed_ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            surface_intersects.push(Intersection { shape: self, t: t1 });
        }

        let mut cap_intersects = self.intersect_caps(&transformed_ray);
        surface_intersects.append(&mut cap_intersects);
        surface_intersects
    }

    fn normal_at(&self, t: Tuple) -> Tuple {
        let object_point = self.get_inverse_transform() * &t;

        let dist = object_point.x.powi(2) + object_point.z.powi(2);

        let object_normal = if dist < 1.0 && object_point.y >= self.maximum - EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && object_point.y <= self.minimum + EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            Tuple::vector(object_point.x, 0.0, object_point.z)
        };

        // convert the normal vector in object space back to world space
        object_space_to_world_space(self, &object_normal)
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
