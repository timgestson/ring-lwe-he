use core::fmt::Debug;
use core::ops::{Add, Mul, Rem, Sub, SubAssign};

pub trait Field: 'static + Clone + SubAssign +
    Add<Self, Output = Self> + Mul<Self, Output = Self> + Sub<Self, Output = Self> + Copy + Debug
{
    const MODULUS: Self;
    const ZERO: Self;
    const ONE: Self;

    fn is_zero(self) -> bool;
    fn inv(self) -> Self;
    fn neg(self) -> Self;
}
