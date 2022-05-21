use std::sync::atomic::AtomicUsize;

use crate::{
    draw::material::Material,
    math::{
        matrix::Matrix,
        ray::Ray,
        tuples::Tuple,
        utils::{f64_eq, EPSILON},
    },
    scene::world::World,
};

// atomic counter to ensure each shape in the scene will have a unique id
pub static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub shape: &'a dyn Intersectable,
    pub t: f64,
    pub u: Option<f64>,
    pub v: Option<f64>,
}

impl<'a> Intersection<'a> {
    pub fn new(shape: &'a dyn Intersectable, t: f64) -> Self {
        Intersection {
            shape,
            t,
            u: None,
            v: None,
        }
    }

    pub fn new_uv(shape: &'a dyn Intersectable, t: f64, u: f64, v: f64) -> Self {
        Intersection {
            shape,
            t,
            u: Some(u),
            v: Some(v),
        }
    }
}

pub trait Intersectable: Sync + Send {
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, t: Tuple, hit: Intersection) -> Tuple;
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, mat: Material);
    fn get_transform(&self) -> &Matrix;
    fn get_inverse_transform(&self) -> &Matrix;
    fn get_inverse_transform_transpose(&self) -> &Matrix;
    fn get_id(&self) -> usize; // random number to uniquely identify this shape
    fn get_parent_id(&self) -> Option<usize>;
    fn set_parent_id(&mut self, id: usize);

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let inv = self.get_inverse_transform();
        let r = ray.apply_transform(inv);
        self.local_intersect(&r)
    }

    fn get_object_by_id(&self, _id: usize) -> Option<&dyn Intersectable> {
        None
    }

    fn world_to_object(&self, point: Tuple, w: &World) -> Tuple {
        let object_point = match self.get_parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.world_to_object(point, w)
            }
            None => point,
        };

        self.get_inverse_transform() * &object_point
    }

    fn normal_to_world(&self, normal: Tuple, w: &World) -> Tuple {
        assert!(normal.is_vector());
        let mut norm = self.get_inverse_transform_transpose() * &normal;
        norm.w = 0.0;
        let world_normal = norm.normalize();

        match self.get_parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.normal_to_world(world_normal, w)
            }
            None => world_normal,
        }
    }

    fn normal_at(&self, point: Tuple, hit: Intersection, w: Option<&World>) -> Tuple {
        match w {
            Some(w) => {
                let local_point = self.world_to_object(point, w);
                let local_normal = self.local_normal_at(local_point, hit);
                self.normal_to_world(local_normal, w)
            }
            None => {
                let local_point = self.get_inverse_transform() * &point;
                let local_normal = self.local_normal_at(local_point, hit);
                let mut transformed = self.get_inverse_transform_transpose() * &local_normal;
                transformed.w = 0.0;
                transformed.normalize()
            }
        }
    }
}

impl PartialEq for dyn Intersectable {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

/*
    Given a list of intersections determine which one would be visible,

    For our purposes this is the intersection with smallest non negative value,
    i.e. the one closet to the camera, a negative value indicates the intersection
    happened behind the camera and hence should not be shown
*/
pub fn hit<'a>(intersections: &[Intersection<'a>]) -> Option<Intersection<'a>> {
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
    pub over_point: Tuple, // a point that lies just above the intersected surface
    pub under_point: Tuple, // a point that lies just below the intersected surface
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool, // if the ray was cast from inside the object
    pub n1: f64,
    pub n2: f64,
}

fn hits_equal(a: &Intersection, b: &Intersection) -> bool {
    a.shape.get_id() == b.shape.get_id() && f64_eq(a.t, b.t)
}

pub fn prepare_computations<'a>(
    hit: &'a Intersection,
    ray: &'a Ray,
    intersections: &[Intersection],
    world: Option<&World>,
) -> Computations<'a> {
    let point = ray.position(hit.t);
    let mut normalv = hit.shape.normal_at(point, *hit, world);
    let eyev = -ray.direction;
    let inside = normalv.dot(&eyev) < 0.0;

    if inside {
        normalv *= -1.0;
    }

    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;

    let reflectv = ray.direction.reflect(&normalv);

    // record what objects have been entered but not yet exited
    let mut containers: Vec<&dyn Intersectable> = vec![];
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    for i in intersections {
        // we have found the hits entrance into the refractive object, the index must be the last container we saw
        // if there are no more objects then we have nothing to collide with, set index to 1
        if hits_equal(hit, i) {
            n1 = match containers.last() {
                Some(container) => container.get_material().refractive_index,
                None => 1.0,
            }
        }

        // if the object is already in our list, then the intersection we just processed must be leaving the object
        // otherwise we are entering the object and need to keep it in the list
        match containers
            .iter()
            .position(|x| x.get_id() == i.shape.get_id())
        {
            Some(index) => {
                containers.remove(index);
            }
            None => containers.push(i.shape),
        };

        // ths hits exit from the refractive object
        if hits_equal(hit, i) {
            n2 = match containers.last() {
                Some(container) => container.get_material().refractive_index,
                None => 1.0,
            };
            break;
        }
    }

    Computations {
        t: hit.t,
        object: hit.shape,
        point,
        over_point,
        under_point,
        eyev,
        normalv,
        reflectv,
        inside,
        n1,
        n2,
    }
}

#[cfg(test)]
mod test {

    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::{
        math::{matrix::Matrix, utils::f64_eq},
        scene::world::World,
        shapes::{plane::Plane, sphere::Sphere},
    };

    use super::*;

    #[test]
    fn normal_vector_normalized() {
        let s = Sphere::new(None);
        let dummy_hit: Intersection = Intersection::new(&s, 0.0);
        let n = s.normal_at(Tuple::point(0.5, 1.0, 0.33), dummy_hit, None);
        assert!(n == n.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Sphere::new(Some(Matrix::translation(0.0, 1.0, 0.0)));
        let dummy_hit: Intersection = Intersection::new(&s, 0.0);
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2), dummy_hit, None);
        assert!(n == Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Sphere::new(Some(
            &Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(PI / 5.0),
        ));
        let dummy_hit: Intersection = Intersection::new(&s, 0.0);
        let n = s.normal_at(
            Tuple::point(0.0, (2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0),
            dummy_hit,
            None,
        );
        assert!(n == Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn hit_with_positive_t() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let v = vec![i1, i2];
        let i = hit(&v).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_with_negative_t() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 1.0);
        let v = vec![i1, i2];
        let i = hit(&v).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn no_hit_with_all_negatives() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, -2.0);
        let v = vec![i1, i2];
        let i = hit(&v);
        assert!(i.is_none());
    }

    #[test]
    fn hit_with_positive_and_negative_t() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 5.0);
        let i3 = Intersection::new(&s, 7.0);
        let i4 = Intersection::new(&s, -2.0);
        let v = vec![i1, i2, i3, i4];
        let i = hit(&v).unwrap();
        assert_eq!(i.t, 5.0);
    }

    #[test]
    fn prepare_computations_intersect_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let intersections = s.intersect(&r);
        let comps = prepare_computations(&intersections[0], &r, &intersections, None);
        assert!(f64_eq(comps.t, intersections[0].t));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert!(!comps.inside);
    }

    #[test]
    fn prepare_computations_intersect_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(None);
        let intersections = s.intersect(&r);
        let comps = prepare_computations(&intersections[1], &r, &intersections, None);
        assert!(f64_eq(comps.t, intersections[1].t));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(Matrix::translation(0.0, 0.0, 1.0)));

        let intersections = s.intersect(&r);
        let comps = prepare_computations(&intersections[0], &r, &intersections, None);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn pre_compute_reflect_vector() {
        let s = Plane::new(None);
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, (2.0_f64).sqrt() / -2.0, (2.0_f64).sqrt() / 2.0),
        );
        let intersections = s.intersect(&r);
        assert!(intersections.len() == 1);
        let comps = prepare_computations(&intersections[0], &r, &intersections, None);
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_of_intersections() {
        let mut a = Sphere::new_glass_sphere(Some(Matrix::scaling(2., 2., 2.)));
        a.material.refractive_index = 1.5;

        let mut b = Sphere::new_glass_sphere(Some(Matrix::translation(0., 0., -0.25)));
        b.material.refractive_index = 2.0;

        let mut c = Sphere::new_glass_sphere(Some(Matrix::translation(0., 0., 0.25)));
        c.material.refractive_index = 2.5;

        let mut w = World::new();
        w.objects = vec![Box::new(a), Box::new(b), Box::new(c)];
        let ray = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&ray);

        let expected_results = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for i in 0..xs.len() {
            let comps = prepare_computations(&xs[i], &ray, &xs, None);
            assert!(f64_eq(comps.n1, expected_results[i].0));
            assert!(f64_eq(comps.n2, expected_results[i].1));
        }
    }

    #[test]
    fn objects_id_is_unique() {
        let s1 = Sphere::new(None);
        let s2 = Sphere::new(None);
        assert_ne!(s1.get_id(), s2.get_id());
        assert_eq!(s1.get_id(), s1.get_id());
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new_glass_sphere(Some(Matrix::translation(0., 0., 1.)));
        let xs = s.intersect(&r);
        let comps = prepare_computations(&xs[0], &r, &xs, None);
        assert!(comps.under_point.z > EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }
}
