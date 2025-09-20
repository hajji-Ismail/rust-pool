use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Copy + Add + Sub + Mul + Div {
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
