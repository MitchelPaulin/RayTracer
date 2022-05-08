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
mod stripe_test {
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

// ---- Gradient ----

pub struct Gradient {
    a: Color,
    b: Color,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient {
            a,
            b,
            transform: Matrix::identity(4),
            inv_transform: Matrix::identity(4),
        }
    }
}

impl Pattern for Gradient {
    fn color_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
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
mod gradient_tests {
    use super::*;

    #[test]
    fn gradient_works() {
        let pattern = Gradient::new(Color::white(), Color::black());
        assert_eq!(
            pattern.color_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Tuple::point(0.50, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Tuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}

// --------

// ---- Rings ----

pub struct Rings {
    a: Color,
    b: Color,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Rings {
    pub fn new(a: Color, b: Color) -> Rings {
        Rings {
            a,
            b,
            transform: Matrix::identity(4),
            inv_transform: Matrix::identity(4),
        }
    }
}

impl Pattern for Rings {
    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() as i64 % 2 == 0 {
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

// --------

// ---- Checkered ----

pub struct Checkered {
    a: Color,
    b: Color,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Checkered {
    pub fn new(a: Color, b: Color) -> Checkered {
        Checkered {
            a,
            b,
            transform: Matrix::identity(4),
            inv_transform: Matrix::identity(4),
        }
    }
}

impl Pattern for Checkered {
    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i64 % 2 == 0 {
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

// --------
