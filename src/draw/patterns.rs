use crate::math::{matrix::Matrix, tuples::Tuple};

use super::color::Color;

pub trait Pattern: Sync + Send {
    fn color_at(&self, point: &Tuple) -> Color;
    fn transform(&self) -> &Matrix;
    fn inverse_transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);
}

// --- Solid ----
#[derive(Clone)]
pub struct Solid {
    c: Color,
    transform: Matrix,
}

impl Solid {
    pub fn new(c: Color) -> Solid {
        Solid {
            c,
            transform: Matrix::identity(4),
        }
    }
}

impl Pattern for Solid {
    fn color_at(&self, _: &Tuple) -> Color {
        self.c
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, _: Matrix) {
        // a transform on a solid pattern does nothing
    }

    fn inverse_transform(&self) -> &Matrix {
        // transforming a solid pattern does nothing
        &self.transform
    }
}
// --------

// ---- Stripe ----
#[derive(Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe {
            a,
            b,
            transform: Matrix::identity(4),
            inv_transform: Matrix::identity(4),
        }
    }
}

impl Pattern for Stripe {
    fn color_at(&self, point: &Tuple) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.inv_transform = self.transform.inverse();
    }

    fn inverse_transform(&self) -> &Matrix {
        &self.inv_transform
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stripe_test() {
        let p = Stripe::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&Tuple::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&Tuple::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&Tuple::point(-0.1, 0.0, 0.0)), Color::black());
    }
}

// --------
