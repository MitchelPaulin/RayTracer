use std::sync::atomic::Ordering;

use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

use super::intersect::{
    object_space_to_world_space, transform_ray_to_object_space, Intersectable, Intersection,
    OBJECT_COUNTER,
};

pub struct Cube {
    id: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    inverse_transform_transpose: Matrix,
    pub material: Material,
}

impl Cube {
    pub fn new(transform: Option<Matrix>) -> Cube {
        let id = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);
        match transform {
            Some(matrix) => {
                assert_eq!(matrix.size, 4);
                // cache some matrices so we don't need to calculate it every time
                let inverse = matrix.inverse();
                let mut inv_transpose = matrix.inverse();
                inv_transpose.transpose();
                Cube {
                    transform: matrix,
                    inverse_transform: inverse,
                    inverse_transform_transpose: inv_transpose,
                    material: Material::default_material(),
                    id,
                }
            }
            None => Cube {
                transform: Matrix::identity(4),
                inverse_transform: Matrix::identity(4),
                inverse_transform_transpose: Matrix::identity(4),
                material: Material::default_material(),
                id,
            },
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin;
    let mut tmax;
    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f64::INFINITY;
        tmax = tmax_numerator * f64::INFINITY;
    }

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }
    (tmin, tmax)
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = transform_ray_to_object_space(self, ray);

        let x = check_axis(transformed_ray.origin.x, transformed_ray.direction.x);
        let y = check_axis(transformed_ray.origin.y, transformed_ray.direction.y);
        let z = check_axis(transformed_ray.origin.z, transformed_ray.direction.z);

        let tmin = [x.0, y.0, z.0].iter().copied().fold(f64::NAN, f64::max);
        let tmax = [x.1, y.1, z.1].iter().copied().fold(f64::NAN, f64::min);

        if tmin > tmax {
            return vec![];
        }

        vec![
            Intersection {
                shape: self,
                t: tmin,
            },
            Intersection {
                shape: self,
                t: tmax,
            },
        ]
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        // convert form world space to object space
        let object_point = self.get_inverse_transform() * &world_point;

        let maxc = [
            object_point.x.abs(),
            object_point.y.abs(),
            object_point.z.abs(),
        ]
        .iter()
        .copied()
        .fold(f64::NAN, f64::max);

        let object_normal = if maxc == object_point.x.abs() {
            Tuple::vector(object_point.x, 0.0, 0.0)
        } else if maxc == object_point.y.abs() {
            Tuple::vector(0.0, object_point.y, 0.0)
        } else {
            Tuple::vector(0.0, 0.0, object_point.z)
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

#[cfg(test)]
mod test {
    use crate::math::utils::f64_eq;

    use super::*;

    #[test]
    fn intersect() {
        let c = Cube::new(None);
        let rays = vec![
            Ray::new(Tuple::point(5.0, 0.5, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
            Ray::new(Tuple::point(-5.0, 0.5, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            Ray::new(Tuple::point(0.5, 5.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            Ray::new(Tuple::point(0.5, -5.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            Ray::new(Tuple::point(0.5, 0.0, 5.0), Tuple::vector(0.0, 0.0, -1.0)),
            Ray::new(Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)),
            Ray::new(Tuple::point(0.0, 0.5, 0.0), Tuple::vector(0.0, 0.0, 1.0)),
        ];

        let ts = vec![
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (-1.0, 1.0),
        ];

        for i in 0..rays.len() {
            let xs = c.intersect(&rays[i]);
            assert_eq!(xs.len(), 2);
            assert!(f64_eq(xs[0].t, ts[i].0));
            assert!(f64_eq(xs[1].t, ts[i].1));
        }
    }

    #[test]
    fn intersection_miss() {
        let c = Cube::new(None);
        let rays = vec![
            Ray::new(
                Tuple::point(-2.0, 0.0, 0.0),
                Tuple::vector(0.2673, 0.5345, 0.8018),
            ),
            Ray::new(
                Tuple::point(0.0, -2.0, 0.0),
                Tuple::vector(0.8018, 0.2673, 0.5345),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(0.5345, 0.8018, 0.2673),
            ),
            Ray::new(Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
            Ray::new(Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
            Ray::new(Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        for i in 0..rays.len() {
            let xs = c.intersect(&rays[i]);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn normal_test() {
        let c = Cube::new(None);
        let points = vec![
            Tuple::point(1., 0.5, -0.8),
            Tuple::point(-1., -0.2, 0.9),
            Tuple::point(-0.4, 1., -0.1),
            Tuple::point(0.3, -1., -0.7),
            Tuple::point(-0.6, 0.3, 1.),
            Tuple::point(0.4, 0.4, -1.),
            Tuple::point(1., 1., 1.),
            Tuple::point(-1., -1., -1.),
        ];

        let normals = vec![
            Tuple::vector(1., 0., 0.),
            Tuple::vector(-1., 0., 0.),
            Tuple::vector(0., 1., 0.),
            Tuple::vector(0., -1., 0.),
            Tuple::vector(0., 0., 1.),
            Tuple::vector(0., 0., -1.),
            Tuple::vector(1., 0., 0.),
            Tuple::vector(-1., 0., 0.),
        ];

        for i in 0..points.len() {
            let normal = c.normal_at(points[i]);
            assert_eq!(normal, normals[i]);
        }
    }
}
