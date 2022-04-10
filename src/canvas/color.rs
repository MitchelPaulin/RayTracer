use std::{fmt, ops};

use crate::math::utils::f32_eq;

#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, val: Self) -> Self {
        Self {
            r: self.r + val.r,
            g: self.g + val.g,
            b: self.b + val.b,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, val: Self) -> Self {
        Self {
            r: self.r - val.r,
            g: self.g - val.g,
            b: self.b - val.b,
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl ops::Div<f32> for Color {
    type Output = Self;
    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            Color::clamp(self.r),
            Color::clamp(self.g),
            Color::clamp(self.b)
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f32_eq(self.r, other.r) && f32_eq(self.g, other.g) && f32_eq(self.b, other.b)
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    fn clamp(val: f32) -> u8 {
        if val < 0.0 {
            0
        } else {
            // max u8 is 255 so no truncation needed
            (val * 255.0) as u8
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clamp_test() {
        assert_eq!(Color::clamp(-1.0), 0);
        assert_eq!(Color::clamp(1.0), 255);
        assert_eq!(Color::clamp(100.0), 255);
        assert_eq!(Color::clamp(0.5), 127);
    }

    #[test]
    fn color_create() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.r, 0.1);
        assert_eq!(c.g, 0.2);
        assert_eq!(c.b, 0.3);
    }

    #[test]
    fn adding_colors() {
        let mut c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = Color::new(1.6, 0.7, 1.0);

        let add = c1 + c2;
        assert!(add == res);

        c1 += c2;
        assert!(res == c1);
    }

    #[test]
    fn subtracting_colors() {
        let mut c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = Color::new(0.2, 0.5, 0.5);

        let sub = c1 - c2;
        assert!(res == sub);

        c1 -= c2;
        assert!(res == c1);
    }

    #[test]
    fn scale_colors() {
        let mut c = Color::new(0.2, 0.3, 0.4);
        let res = Color::new(0.4, 0.6, 0.8);

        let mul = c * 2.0;
        assert!(res == mul);

        c *= 2.0;
        assert!(res == c);

        c /= 2.0;
        c = c / 0.5;
        assert!(res == c);
    }

    #[test]
    fn mul_colors() {
        let mut c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let res = Color::new(0.9, 0.2, 0.04);

        let mul = c1 * c2;
        assert!(res == mul);

        c1 *= c2;
        assert!(res == c1);
    }
}
