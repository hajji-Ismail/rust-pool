
use std::{ops::{Add, Div, Mul, Sub}};
pub trait Scalar: Copy + Add + Sub + Mul + Div {
    fn zero() -> Self;
    fn one() -> Self;
}
impl Scalar for i32 {
  
    fn zero() -> Self {
        0
    }
    fn one() -> Self{
        1
    }
}
impl Scalar for i8 {
  
    fn zero() -> Self {
        0
    }
    fn one() -> Self{
        1
    }
}
impl Scalar for i16 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for i64 {

    fn zero() -> Self {
        0
    }
    fn one() -> Self{
        1
    }
}
impl Scalar for i128 {
   
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for u8 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for u16 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for u128 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}
impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}
#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl <T: Scalar> Matrix<T> {
	pub fn new() -> Matrix<T> {
       return  Self(vec![vec![]]);
    
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
    Matrix(vec![vec![T::zero();col];row])

	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Self::zero(n, n);
        for i in 0.. m.0.len() {
            m.0[i][i] = T::one();



        }
      m

	}
}
impl<T> Add for Matrix<T>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut m = Matrix::zero(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                m.0[i][j] = self.0[i][j] + rhs.0[i][j]; 
            }
        }
        Some(m)
    }
}

impl <T: std::ops::Sub<Output = T>+ Scalar> Sub for Matrix<T>{
       type Output =   Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
               if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
     let rows = self.0.len();
        let cols = self.0[0].len();
        let mut m = Matrix::zero(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                m.0[i][j] = self.0[i][j] - rhs.0[i][j];
            }
        }
      Some(m)
    }

}