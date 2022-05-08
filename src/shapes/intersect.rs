use crate::{
    draw::material::Material,
    math::{matrix::Matrix, ray::Ray, tuples::Tuple, utils::EPSILON},
};

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub shape: &'a dyn Intersectable,
    pub t: f64,
}

pub trait Intersectable: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, t: Tuple) -> Tuple;
    fn get_material(&self) -> &Material;
    fn get_transform(&self) -> &Matrix;
    fn get_inverse_transform(&self) -> &Matrix;
    fn get_inverse_transform_transpose(&self) -> &Matrix;
}

/*
    Given a list of intersections determine which one would be visible,

    For our purposes this is the intersection with smallest non negative value,
    i.e. the one closet to the camera, a negative value indicates the intersection
    happened behind the camera and hence should not be shown
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

pub fn transform_ray_to_object_space(shape: &dyn Intersectable, ray: &Ray) -> Ray {
    /*
        Rather than transforming the sphere we can transform the ray by the inverse of the sphere transform,
        this has the same effect on the resulting intersections and allows us to assume were still
        working with a unit sphere
    */
    let inv = shape.get_inverse_transform();
    ray.apply_transform(inv)
}

pub fn object_space_to_world_space(shape: &dyn Intersectable, object_normal: &Tuple) -> Tuple {
    let mut world_normal = shape.get_inverse_transform_transpose() * object_normal;
    world_normal.w = 0.0;
    world_normal.normalize()
}

/*
    Pre-compute some values related to the intersection for later use
*/
pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a dyn Intersectable,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool, // if the ray was cast from inside the object
}

pub fn prepare_computations<'a>(intersection: &'a Intersection, ray: &'a Ray) -> Computations<'a> {
    let point = ray.position(intersection.t);
    let mut normalv = intersection.shape.normal_at(point);
    let eyev = -ray.direction;
    let inside = normalv.dot(&eyev) < 0.0;
    let over_point = point + normalv * EPSILON;

    if inside {
        normalv *= -1.0;
    }

    Computations {
        t: intersection.t,
        object: intersection.shape,
        point,
        over_point,
        eyev,
        normalv,
        inside,
    }
}

#[cfg(test)]
mod test {

    use crate::{
        math::{matrix::Matrix, utils::f64_eq},
        shapes::sphere::Sphere,
    };

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

    #[test]
    fn prepare_computations_intersect_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let intersection = s.intersect(&r)[0];
        let comps = prepare_computations(&intersection, &r);
        assert!(f64_eq(comps.t, intersection.t));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert!(!comps.inside);
    }

    #[test]
    fn prepare_computations_intersect_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let intersection = s.intersect(&r)[1];
        let comps = prepare_computations(&intersection, &r);
        assert!(f64_eq(comps.t, intersection.t));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(Matrix::translation(0.0, 0.0, 1.0)));
        let i = s.intersect(&r)[0];
        let comps = prepare_computations(&i, &r);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
