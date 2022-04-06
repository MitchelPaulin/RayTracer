use std::{fmt, ops};

#[derive(Clone, Copy, PartialEq)]
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

impl Color {
    pub fn color(r: f32, g: f32, b: f32) -> Color {
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
    use crate::utils::f32_eq;

    #[test]
    fn clamp_test() {
        assert_eq!(Color::clamp(-1.0), 0);
        assert_eq!(Color::clamp(1.0), 255);
        assert_eq!(Color::clamp(100.0), 255);
        assert_eq!(Color::clamp(0.5), 127);
    }

    #[test]
    fn color_create() {
        let c = Color::color(0.1, 0.2, 0.3);
        assert_eq!(c.r, 0.1);
        assert_eq!(c.g, 0.2);
        assert_eq!(c.b, 0.3);
    }

    #[test]
    fn adding_colors() {
        let mut c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        let res = Color::color(1.6, 0.7, 1.0);

        let add = c1 + c2;
        assert!(f32_eq(res.r, add.r));
        assert!(f32_eq(res.g, add.g));
        assert!(f32_eq(res.b, add.b));

        c1 += c2;
        assert!(f32_eq(res.r, c1.r));
        assert!(f32_eq(res.g, c1.g));
        assert!(f32_eq(res.b, c1.b));
    }

    #[test]
    fn subtracting_colors() {
        let mut c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        let res = Color::color(0.2, 0.5, 0.5);

        let sub = c1 - c2;
        assert!(f32_eq(res.r, sub.r));
        assert!(f32_eq(res.g, sub.g));
        assert!(f32_eq(res.b, sub.b));

        c1 -= c2;
        assert!(f32_eq(res.r, c1.r));
        assert!(f32_eq(res.g, c1.g));
        assert!(f32_eq(res.b, c1.b));
    }

    #[test]
    fn scale_colors() {
        let mut c = Color::color(0.2, 0.3, 0.4);
        let res = Color::color(0.4, 0.6, 0.8);

        let mul = c * 2.0;
        assert!(f32_eq(res.r, mul.r));
        assert!(f32_eq(res.g, mul.g));
        assert!(f32_eq(res.b, mul.b));

        c *= 2.0;
        assert!(f32_eq(res.r, c.r));
        assert!(f32_eq(res.g, c.g));
        assert!(f32_eq(res.b, c.b));

        c /= 2.0;
        c = c / 0.5;
        assert!(f32_eq(res.r, c.r));
        assert!(f32_eq(res.g, c.g));
        assert!(f32_eq(res.b, c.b));
    }

    #[test]
    fn mul_colors() {
        let mut c1 = Color::color(1.0, 0.2, 0.4);
        let c2 = Color::color(0.9, 1.0, 0.1);
        let res = Color::color(0.9, 0.2, 0.04);

        let mul = c1 * c2;
        assert!(f32_eq(res.r, mul.r));
        assert!(f32_eq(res.g, mul.g));
        assert!(f32_eq(res.b, mul.b));

        c1 *= c2;
        assert!(f32_eq(res.r, c1.r));
        assert!(f32_eq(res.g, c1.g));
        assert!(f32_eq(res.b, c1.b));
    }
}
