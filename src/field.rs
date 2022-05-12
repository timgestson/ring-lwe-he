use core::fmt::Debug;
use core::ops::{Add, Mul, Sub, SubAssign};

pub trait Field:
    'static
    + Clone
    + SubAssign
    + From<i32>
    + Into<i32>
    + Into<f64>
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Copy
    + Debug
{
    const MODULUS: Self;
    const ZERO: Self;
    const ONE: Self;

    fn new(int: u64) -> Self;
    fn is_zero(self) -> bool;
    fn inv(self) -> Self;
    fn neg(self) -> Self;
}