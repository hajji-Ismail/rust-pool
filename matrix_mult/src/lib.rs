use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}


macro_rules! impl_scalar {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                fn zero() -> Self { 0 as $t }
                fn one() -> Self { 1 as $t }
            }
        )*
    };
}

impl_scalar!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        (0..self.0.len()).map(|i| self.0[i][n]).collect()
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let m = self.number_of_rows();
        let n = self.number_of_cols();
        let p = rhs.number_of_cols();

        let mut result = vec![vec![T::zero(); p]; m];

        for i in 0..m {
            for j in 0..p {
                let mut sum = T::zero();
                for k in 0..n {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
