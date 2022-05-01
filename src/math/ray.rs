use super::{matrix::Matrix, tuples::Tuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Ray {
        let translation = Matrix::translation(x, y, z);

        self.apply_transform(&translation)
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Ray {
        let scale = Matrix::scaling(x, y, z);

        self.apply_transform(&scale)
    }

    pub fn apply_transform(&self, transform: &Matrix) -> Ray {
        Ray {
            origin: transform * &self.origin,
            direction: transform * &self.direction,
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert!(ray.origin == origin);
        assert!(ray.direction == direction);
    }

    #[test]
    fn position_test() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert!(ray.position(0.0) == Tuple::point(2.0, 3.0, 4.0));
        assert!(ray.position(1.0) == Tuple::point(3.0, 3.0, 4.0));
    }

    #[test]
    fn translate_test() {
        let r = Ray {
            origin: Tuple::point(1.0, 2.0, 3.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };

        let res = r.translate(3.0, 4.0, 5.0);

        assert!(res.origin == Tuple::point(4.0, 6.0, 8.0));
        assert!(res.direction == Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_test() {
        let r = Ray {
            origin: Tuple::point(1.0, 2.0, 3.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };

        let res = r.scale(2.0, 3.0, 4.0);

        assert!(res.origin == Tuple::point(2.0, 6.0, 12.0));
        assert!(res.direction == Tuple::vector(0.0, 3.0, 0.0));
    }
}
