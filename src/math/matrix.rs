use std::ops;

use crate::math::tuples::Tuple;

use super::utils::f32_eq;

/*
    A specialized matrix library for only square matrices
*/

#[derive(Clone, Debug)]
pub struct Matrix {
    pub size: usize,
    pub matrix: Vec<Vec<f32>>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for i in 0..self.size {
            for j in 0..self.size {
                if !f32_eq(self.matrix[i][j], other.matrix[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.size, rhs.size);

        let mut res = Matrix {
            size: self.size,
            matrix: vec![vec![0.0; self.size]; self.size],
        };

        for row in 0..self.size {
            for col in 0..self.size {
                for i in 0..self.size {
                    res.matrix[row][col] += self.matrix[row][i] * rhs.matrix[i][col];
                }
            }
        }

        res
    }
}

impl ops::Mul<&Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Tuple {
        assert_eq!(self.size, 4);
        let mut res = [0.0; 4];
        for (row, r) in res.iter_mut().enumerate().take(self.size) {
            *r += self.matrix[row][0] * rhs.x
                + self.matrix[row][1] * rhs.y
                + self.matrix[row][2] * rhs.z
                + self.matrix[row][3] * rhs.w;
        }

        Tuple::new(res[0], res[1], res[2], res[3])
    }
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        Matrix {
            size,
            matrix: vec![vec![0.0; size]; size],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.matrix[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, val: f32) {
        self.matrix[i][j] = val;
    }

    pub fn identity(size: usize) -> Matrix {
        let mut m = Matrix::new(size);
        for i in 0..size {
            m.matrix[i][i] = 1.0;
        }
        m
    }

    pub fn transpose(&mut self) {
        for n in 0..self.size - 1 {
            for m in n + 1..self.size {
                let temp = self.matrix[n][m];
                self.matrix[n][m] = self.matrix[m][n];
                self.matrix[m][n] = temp;
            }
        }
    }

    pub fn determinant(&self) -> f32 {
        if self.size == 2 {
            return self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0];
        }
        let mut det = 0.0;

        for col in 0..self.size {
            det += self.matrix[0][col] * self.cofactor(0, col);
        }

        det
    }

    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix {
        let mut ret = self.clone();
        ret.matrix.remove(row);
        for i in 0..self.size - 1 {
            ret.matrix[i].remove(col);
        }
        ret.size -= 1;
        ret
    }

    pub fn minor(&self, i: usize, j: usize) -> f32 {
        let m = self.sub_matrix(i, j);
        m.determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        let m = self.minor(i, j);
        if (i + j) & 1 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn inverse(&self) -> Matrix {
        let mut inverse = self.clone();

        let det = self.determinant();
        assert_ne!(det, 0.0); // is matrix invertible

        for n in 0..self.size {
            for m in 0..self.size {
                let c = self.cofactor(n, m);
                inverse.matrix[m][n] = c / det;
            }
        }

        inverse
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::identity(4);
        m.matrix[0][3] = x;
        m.matrix[1][3] = y;
        m.matrix[2][3] = z;
        m
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::new(4);
        m.matrix[0][0] = x;
        m.matrix[1][1] = y;
        m.matrix[2][2] = z;
        m.matrix[3][3] = 1.0;
        m
    }

    pub fn rotation_x(radians: f32) -> Matrix {
        let mut m = Matrix::identity(4);

        let cos = radians.cos();
        let sin = radians.sin();

        m.matrix[1][1] = cos;
        m.matrix[2][2] = cos;
        m.matrix[1][2] = -sin;
        m.matrix[2][1] = sin;

        m
    }

    pub fn rotation_y(radians: f32) -> Matrix {
        let mut m = Matrix::identity(4);

        let cos = radians.cos();
        let sin = radians.sin();

        m.matrix[0][0] = cos;
        m.matrix[2][2] = cos;
        m.matrix[0][2] = sin;
        m.matrix[2][0] = -sin;

        m
    }

    pub fn rotation_z(radians: f32) -> Matrix {
        let mut m = Matrix::identity(4);

        let cos = radians.cos();
        let sin = radians.sin();

        m.matrix[0][0] = cos;
        m.matrix[1][1] = cos;
        m.matrix[0][1] = -sin;
        m.matrix[1][0] = sin;

        m
    }

    pub fn shear(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix {
        let mut m = Matrix::identity(4);

        m.matrix[0][1] = x_y;
        m.matrix[0][2] = x_z;
        m.matrix[1][0] = y_x;
        m.matrix[1][2] = y_z;
        m.matrix[2][0] = z_x;
        m.matrix[2][1] = z_y;

        m
    }
}

#[cfg(test)]
mod test {
    use std::{f32::consts::PI, vec};

    use super::*;

    #[test]
    fn equality() {
        let m1 = Matrix::new(3);
        let m2 = Matrix::new(3);

        assert!(m1 == m2);
        assert!(m1 == m1);
    }

    #[test]
    fn multiply_matrix() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];

        let mut B = Matrix::new(4);
        B.matrix = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ];

        let mut res = Matrix::new(4);
        res.matrix = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ];

        let C = &A * &B;
        assert!(C == res);
    }

    #[test]
    fn multiply_tuple() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let t = Tuple::point(1.0, 2.0, 3.0);
        let res = Tuple::point(18.0, 24.0, 33.0);

        let At = &A * &t;

        assert!(res == At);
    }

    #[test]
    fn identity() {
        let I = Matrix::identity(4);
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let B = &A * &I;
        assert!(A == B);
    }

    #[test]
    fn transpose() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];

        let mut res = Matrix::new(4);
        res.matrix = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];

        A.transpose();
        assert!(A == res);
    }

    #[test]
    fn determinant_2x2() {
        let mut m = Matrix::new(2);
        m.matrix = vec![vec![1.0, 5.0], vec![-3.0, 2.0]];
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn determinant_3x3() {
        let mut A = Matrix::new(3);
        A.matrix = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ];

        assert_eq!(A.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ];

        assert_eq!(A.determinant(), -4071.0);
    }

    #[test]
    fn submatrix_3x3() {
        let mut A = Matrix::new(3);
        A.matrix = vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ];

        let mut res = Matrix::new(2);
        res.matrix = vec![vec![-3.0, 2.0], vec![0.0, 6.0]];

        let sA = A.sub_matrix(0, 2);
        assert!(sA == res);
    }

    #[test]
    fn submatrix_4x4() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ];

        let mut res = Matrix::new(3);
        res.matrix = vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ];

        let sA = A.sub_matrix(2, 1);
        assert!(sA == res);
    }

    #[test]
    fn minors_3x3() {
        let mut A = Matrix::new(3);
        A.matrix = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        assert_eq!(A.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3() {
        let mut A = Matrix::new(3);
        A.matrix = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        assert_eq!(A.minor(0, 0), A.cofactor(0, 0));
        assert_eq!(A.minor(1, 0), -A.cofactor(1, 0));
    }

    #[test]
    fn inverse_4x4() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ];

        let mut inverse = Matrix::new(4);
        inverse.matrix = vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ];

        let Ai = A.inverse();

        assert!(Ai == inverse);
    }

    #[test]
    fn inverse_4x4_2() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ];

        let mut inverse = Matrix::new(4);
        inverse.matrix = vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ];

        let Ai = A.inverse();

        assert!(Ai == inverse);
    }

    #[test]
    fn sanity_test() {
        let mut A = Matrix::new(4);
        A.matrix = vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ];

        let mut B = Matrix::new(4);
        B.matrix = vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ];

        let C = &A * &B;
        assert!(&C * &B.inverse() == A);
    }

    #[test]
    fn translation_point() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let res = Tuple::point(2.0, 1.0, 7.0);
        assert!(&transform * &p == res);
    }

    #[test]
    fn translation_reverse() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let p = &inverse * &Tuple::point(-3.0, 4.0, 5.0);
        assert!(p == Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_effect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert!(&transform * &v == v);
    }

    #[test]
    fn scaling_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let res = &transform * &p;
        assert!(res == Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let res = &transform * &v;
        assert!(res == Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_reverse() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let res = &inv * &v;
        assert!(res == Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert!(&transform * &p == Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn x_axis_rotation() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        assert!(
            &half_quarter * &p == Tuple::point(0.0, (2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0)
        );
        assert!(&full_quarter * &p == Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn x_inverse_rotation() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert!(&inv * &p == Tuple::point(0.0, (2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / -2.0));
    }

    #[test]
    fn y_axis_rotation() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        assert!(
            &half_quarter * &p == Tuple::point((2.0_f32).sqrt() / 2.0, 0.0, (2.0_f32).sqrt() / 2.0)
        );
        assert!(&full_quarter * &p == Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn z_axis_rotation() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        assert!(
            &half_quarter * &p
                == Tuple::point((2.0_f32).sqrt() / -2.0, (2.0_f32).sqrt() / 2.0, 0.0)
        );
        assert!(&full_quarter * &p == Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear() {
        let p = Tuple::point(2.0, 3.0, 4.0);

        let mut t = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!(&t * &p == Tuple::point(5.0, 3.0, 4.0));

        t = Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert!(&t * &p == Tuple::point(6.0, 3.0, 4.0));

        t = Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert!(&t * &p == Tuple::point(2.0, 5.0, 4.0));

        t = Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!(&t * &p == Tuple::point(2.0, 7.0, 4.0));

        t = Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(&t * &p == Tuple::point(2.0, 3.0, 6.0));

        t = Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(&t * &p == Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let A = Matrix::rotation_x(PI / 2.0);
        let B = Matrix::scaling(5.0, 5.0, 5.0);
        let C = Matrix::translation(10.0, 5.0, 7.0);
        let T = &(&C * &B) * &A;
        assert!(&T * &p == Tuple::point(15.0, 0.0, 7.0));
    }
}
