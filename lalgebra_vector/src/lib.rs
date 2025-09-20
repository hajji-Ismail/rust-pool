use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Copy + Add<Output = Self> + Sub<Output = Self>  + Mul<Output = Self>  + Div<Output = Self>  {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i8 {
    type Item = i8;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i16 {
    type Item = i16;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i128 {
    type Item = i128;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u8 {
    type Item = u8;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u16 {
    type Item = u16;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u128 {
    type Item = u128;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);


impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>; // result is another Vector<T>

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len(){
            return None;
        }
        let data = self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| *a + *b) // need to deref because iter() gives &T
            .collect();
        Some(Vector(data))
    }
}


impl <T: Scalar<Item = T>>Vector<T> {
	pub fn new() -> Self {
        Self(vec![])
	}
pub fn dot(&self, other: &Self) -> Option<T> {
    if self.0.len() == other.0.len() {
        let mut res = T::zero();
        for i in 0..self.0.len() {
            res = res + (self.0[i] * other.0[i]);
        }
        Some(res)
    } else {
        None
    }
}

}