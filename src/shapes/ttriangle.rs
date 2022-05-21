use crate::math::{tuples::Tuple, ray::Ray, utils::EPSILON};

pub trait TTriangle {
    fn e1(&self) -> Tuple;
    fn e2(&self) -> Tuple;
    fn p1(&self) -> Tuple;
}

pub fn moller_trumbore_inner(shape: &dyn TTriangle, ray: &Ray) -> Option<(f64, f64, f64)> {
    // Möller–Trumbore algorithm for triangle-ray intersection

    let dir_cross_e2 = ray.direction.cross(&shape.e2());
    let determinant = shape.e1().dot(&dir_cross_e2);
    if determinant.abs() < EPSILON {
        return None;
    }

    let f = 1.0 / determinant;
    let p1_to_origin = ray.origin - shape.p1();
    let u = f * p1_to_origin.dot(&dir_cross_e2);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let origin_cross_e1 = p1_to_origin.cross(&shape.e1());
    let v = f * ray.direction.dot(&origin_cross_e1);
    if v < 0.0 || (u + v) > 1.0 {
        return None;
    }

    let t = f * shape.e2().dot(&origin_cross_e1);

    Some((t, u, v))
}