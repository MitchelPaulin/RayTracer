use crate::math::tuples::Tuple;

use super::color::Color;

pub trait Pattern: Sync + Send {
    fn get_color_at(&self, point: &Tuple) -> Color;
}

// --- Solid ----
#[derive(Copy, Clone)]
pub struct Solid {
    c: Color,
}

impl Solid {
    pub fn new(c: Color) -> Solid {
        Solid { c }
    }
}

impl Pattern for Solid {
    fn get_color_at(&self, _: &Tuple) -> Color {
        self.c
    }
}
// --------

// ---- Stripe ----
#[derive(Copy, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b }
    }
}

impl Pattern for Stripe {
    fn get_color_at(&self, point: &Tuple) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stripe_test() {
        let p = Stripe::new(Color::white(), Color::black());
        assert_eq!(p.get_color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.get_color_at(&Tuple::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.get_color_at(&Tuple::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.get_color_at(&Tuple::point(-0.1, 0.0, 0.0)), Color::black());
    }
}

// --------
