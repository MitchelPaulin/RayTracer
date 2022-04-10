use crate::math::ray::Ray;

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub shape: &'a dyn Intersect,
    pub t: f32,
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

/*
    Given a list of intersections determine which one would be visible,

    For our purposes this is the intersection with smallest non negative value
*/
pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    if intersections.is_empty() {
        return None;
    }

    let mut front_intersection: Option<Intersection> = None;

    for intersection in intersections.iter().filter(|i| i.t > 0.0) {
        if front_intersection.is_none() || intersection.t < front_intersection.unwrap().t {
            front_intersection = Some(*intersection);
        }
    }

    front_intersection
}

#[cfg(test)]
mod test {

    use crate::{shapes::sphere::Sphere};

    use super::*;

    #[test]
    fn hit_with_positive_t() {
        let s = Sphere::new(None);
        let i1 = Intersection { shape: &s, t: 1.0 };
        let i2 = Intersection { shape: &s, t: 2.0 };
        let i = hit(vec![i1, i2]).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_with_negative_t() {
        let s = Sphere::new(None);
        let i1 = Intersection { shape: &s, t: -1.0 };
        let i2 = Intersection { shape: &s, t: 1.0 };
        let i = hit(vec![i1, i2]).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn no_hit_with_all_negatives() {
        let s = Sphere::new(None);
        let i1 = Intersection { shape: &s, t: -1.0 };
        let i2 = Intersection { shape: &s, t: -2.0 };
        let i = hit(vec![i1, i2]);
        assert!(i.is_none());
    }

    #[test]
    fn hit_with_positive_and_negative_t() {
        let s = Sphere::new(None);
        let i1 = Intersection { shape: &s, t: -1.0 };
        let i2 = Intersection { shape: &s, t: 5.0 };
        let i3 = Intersection { shape: &s, t: 7.0 };
        let i4 = Intersection { shape: &s, t: -2.0 };
        let i = hit(vec![i1, i2, i3, i4]).unwrap();
        assert_eq!(i.t, 5.0);
    }
}
