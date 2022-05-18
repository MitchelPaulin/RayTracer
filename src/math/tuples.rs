use std::ops;

use super::utils::f64_eq;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, val: Self) -> Self {
        let ret = Self {
            x: self.x + val.x,
            y: self.y + val.y,
            z: self.z + val.z,
            w: self.w + val.w,
        };
        assert!(self.is_vector() || self.is_point());
        ret
    }
}

impl ops::AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
        assert!(self.is_vector() || self.is_point());
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, val: Self) -> Self {
        let ret = Tuple {
            x: self.x - val.x,
            y: self.y - val.y,
            z: self.z - val.z,
            w: self.w - val.w,
        };
        assert!(self.is_vector() || self.is_point());
        ret
    }
}

impl ops::SubAssign for Tuple {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
        assert!(self.is_vector() || self.is_point());
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Tuple {
        assert!(self.is_vector());
        Tuple::vector(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<f64> for Tuple {
    fn div_assign(&mut self, rhs: f64) {
        assert!(self.is_vector());
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Tuple {
        assert!(self.is_vector());
        Tuple::vector(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        assert!(self.is_vector());
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        assert!(self.is_vector());
        Tuple {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            w: self.w,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.x, other.x)
            && f64_eq(self.y, other.y)
            && f64_eq(self.z, other.z)
            && f64_eq(self.w, other.w)
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn is_vector(&self) -> bool {
        f64_eq(self.w, 0.0)
    }

    pub fn is_point(&self) -> bool {
        f64_eq(self.w, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        //assert!(self.is_vector());
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        //assert!(self.is_vector());
        let mag = self.magnitude();
        assert!(!f64_eq(mag, 0.0));
        Tuple::vector(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        assert!(self.is_vector());
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        assert!(self.is_vector());
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /*
        Reflect a vector around a given normal vector
    */
    pub fn reflect(&self, normal_vector: &Tuple) -> Tuple {
        assert!(self.is_vector() && normal_vector.is_vector());
        let scalar = *normal_vector * 2.0 * self.dot(normal_vector);
        *self - scalar
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_create() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
        assert!(v.is_vector());
        assert!(!v.is_point());
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn point_create() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
        assert!(p.is_point());
        assert!(!p.is_vector());
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn tuple_equal() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(v == v);

        let v_2 = Tuple::vector(1.0, 2.0, 1.0);
        assert!(v != v_2);
    }

    #[test]
    fn adding_tuples() {
        // vector + vector
        let mut t_1 = Tuple::vector(1.0, 2.0, 3.0);
        let mut t_2 = Tuple::vector(4.0, 5.0, 6.0);
        let mut res = Tuple::vector(5.0, 7.0, 9.0);
        let mut add = t_1 + t_2;
        t_1 += t_2;
        assert!(t_1 == res);
        assert!(add == res);

        // vector + point
        t_1 = Tuple::vector(1.0, 2.0, 3.0);
        t_2 = Tuple::point(4.0, 5.0, 6.0);
        res = Tuple::point(5.0, 7.0, 9.0);
        add = t_1 + t_2;
        t_1 += t_2;
        assert!(t_1 == res);
        assert!(add == res);
    }

    #[test]
    #[should_panic]
    fn adding_points() {
        // point + point
        let mut t_1 = Tuple::point(1.0, 2.0, 3.0);
        let t_2 = Tuple::point(4.0, 5.0, 6.0);
        t_1 += t_2;
    }

    #[test]
    fn subtracting_tuples() {
        // point - vector
        let mut t_1 = Tuple::point(1.0, 2.0, 3.0);
        let mut t_2 = Tuple::vector(4.0, 5.0, -2.0);
        let mut res = Tuple::point(-3.0, -3.0, 5.0);
        let mut sub = t_1 - t_2;
        t_1 -= t_2;
        assert!(t_1 == res);
        assert!(sub == res);

        // vector - vector
        t_1 = Tuple::vector(1.0, 2.0, 3.0);
        t_2 = Tuple::vector(1.0, 3.0, 0.0);
        res = Tuple::vector(0.0, -1.0, 3.0);
        sub = t_1 - t_2;
        t_1 -= t_2;
        assert!(t_1 == res);
        assert!(sub == res);

        // point - point
        t_1 = Tuple::point(1.0, 2.0, 3.0);
        t_2 = Tuple::point(1.0, 3.0, 0.0);
        res = Tuple::vector(0.0, -1.0, 3.0);
        sub = t_1 - t_2;
        t_1 -= t_2;
        assert!(t_1 == res);
        assert!(sub == res);
    }

    #[test]
    #[should_panic]
    fn subtracting_point_from_vector() {
        // vector - point
        let mut t_1 = Tuple::vector(1.0, 2.0, 3.0);
        let t_2 = Tuple::point(4.0, 5.0, 6.0);
        t_1 -= t_2;
    }

    #[test]
    fn negate_vec() {
        let v = Tuple::vector(-1.0, 2.0, 0.0);
        let res = Tuple::vector(1.0, -2.0, 0.0);
        let neg = -v;
        assert!(neg == res);
    }

    #[test]
    #[should_panic]
    fn negate_point() {
        let v = Tuple::point(-1.0, 2.0, 0.0);
        let _ = -v;
    }

    #[test]
    fn scale_test() {
        let mut v = Tuple::vector(-1.0, 2.0, 0.0);
        assert!(v * 1.0 == v);
        v *= 1.0;
        assert!(v == v);

        let res = Tuple::vector(-2.0, 4.0, 0.0);
        assert!(v * 2.0 == res);
        v *= 2.0;
        assert!(v == res);

        assert!((v / 2.0) == (v / 2.0));
        v /= 2.0;
        assert!(v == v);
    }

    #[test]
    fn magnitude() {
        let mut v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        v = Tuple::vector(-1.0, -2.0, -3.0);
        let mag = v.magnitude();
        let expected = 14.0_f64.sqrt();
        assert!(f64_eq(mag, expected))
    }

    #[test]
    fn normalization() {
        let a = Tuple::vector(4.0, 0.0, 0.0);
        let norm = a.normalize();
        let res = Tuple::vector(1.0, 0.0, 0.0);
        assert!(norm == res);

        let a = Tuple::vector(1.0, 2.0, 3.0);
        let norm = a.normalize();
        let res = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert!(norm == res);
    }

    #[test]
    fn dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(Tuple::dot(&a, &b), 20.0);
    }

    #[test]
    fn cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let mut c = Tuple::cross(&a, &b);
        let mut res = Tuple::vector(-1.0, 2.0, -1.0);
        assert!(c == res);

        c = Tuple::cross(&b, &a);
        res = Tuple::vector(1.0, -2.0, 1.0);
        assert!(c == res);
    }

    #[test]
    fn reflect_90() {
        let res = Tuple::vector(1.0, -1.0, 0.0).reflect(&Tuple::vector(0.0, 1.0, 0.0));
        assert!(res == Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_slanted() {
        let res = Tuple::vector(0.0, -1.0, 0.0).reflect(&Tuple::vector(
            (2.0_f64).sqrt() / 2.0,
            (2.0_f64).sqrt() / 2.0,
            0.0,
        ));
        assert!(res == Tuple::vector(1.0, 0.0, 0.0));
    }
}
