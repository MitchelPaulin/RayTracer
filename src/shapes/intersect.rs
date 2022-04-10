use crate::math::ray::Ray;

pub struct Intersection<'a> {
    pub shape: &'a dyn Intersect,
    pub intersection: f32
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}
