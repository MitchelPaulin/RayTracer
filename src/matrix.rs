use std::ops;

use crate::tuples::Tuple;

#[derive(Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<Vec<f32>>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.height != other.height || self.width != other.width {
            return false;
        }
        for i in 0..self.height {
            for j in 0..self.width {
                if self.matrix[i][j] != other.matrix[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.height, rhs.width);
        let mut res = Matrix {
            width: self.width,
            height: rhs.height,
            matrix: vec![vec![0.0; self.width]; rhs.height],
        };

        for row in 0..res.height {
            for col in 0..res.width {
                for i in 0..self.height {
                    res.matrix[row][col] += self.matrix[row][i] * rhs.matrix[i][col];
                }
            }
        }

        res
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
        assert_eq!(self.width, 4);
        let mut res = [0.0; 4];
        for row in 0..self.height {
            res[row] += self.matrix[row][0] * rhs.x;
            res[row] += self.matrix[row][1] * rhs.y;
            res[row] += self.matrix[row][2] * rhs.z;
            res[row] += self.matrix[row][3] * rhs.w;
        }

        Tuple::tuple(res[0], res[1], res[2], res[3])
    }
}

impl Matrix {
    pub fn matrix(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            matrix: vec![vec![0.0; width]; height],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.matrix[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, val: f32) {
        self.matrix[i][j] = val;
    }

    pub fn identity(size: usize) -> Matrix {
        let mut m = Matrix::matrix(size, size);
        for i in 0..size {
            m.matrix[i][i] = 1.0;
        }
        m
    }

    pub fn transpose(&mut self) {
        // this simple in place transpose only works with square matrices
        assert_eq!(self.width, self.height);
        for n in 0..self.width - 1 {
            for m in n + 1..self.width {
                let temp = self.matrix[n][m];
                self.matrix[n][m] = self.matrix[m][n];
                self.matrix[m][n] = temp;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equality() {
        let m1 = Matrix::matrix(3, 4);
        let m2 = Matrix::matrix(3, 4);
        let m3 = Matrix::matrix(3, 2);

        assert!(m1 == m2);
        assert!(m1 == m1);
        assert!(m1 != m3);
    }

    #[test]
    fn multiply_matrix() {
        let mut A = Matrix::matrix(4, 4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];

        let mut B = Matrix::matrix(4, 4);
        B.matrix = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ];

        let mut res = Matrix::matrix(4, 4);
        res.matrix = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ];

        let C = A * B;
        assert!(C == res);
    }

    #[test]
    fn multiply_tuple() {
        let mut A = Matrix::matrix(4, 4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let t = Tuple::point(1.0, 2.0, 3.0);
        let res = Tuple::point(18.0, 24.0, 33.0);

        let At = A * t;

        assert!(res == At);
    }

    #[test]
    fn identity() {
        let I = Matrix::identity(4);
        let mut A = Matrix::matrix(4, 4);
        A.matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let B = A.clone() * I;
        assert!(A == B);
    }

    #[test]
    fn transpose() {
        let mut A = Matrix::matrix(4, 4);
        A.matrix = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];

        let mut res = Matrix::matrix(4, 4);
        res.matrix = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];

        A.transpose();
        assert!(A == res);
    }
}
