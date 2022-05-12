use crate::Field;
use core::fmt::Debug;
use core::ops::{Add, Mul, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Combined(pub u64);

impl Field for Combined {
    const MODULUS: Self = Self((1048576_u64 * 1024_u64));
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn new(num: u64) -> Self {
        Self(num % Self::MODULUS.0)
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn inv(self) -> Self {
        let mut inverse = Combined::ZERO;
        for i in 0..Self::MODULUS.0 {
            inverse = Combined::new(i);
            if self * inverse == Self::ONE {
                break;
            }
        }
        inverse
    }

    fn neg(self) -> Self {
        Self::MODULUS - self
    }
}

impl From<i32> for Combined {
    fn from(int: i32) -> Self {
        let reduced = int as i64 % Self::MODULUS.0 as i64;
        if reduced < 0 {
            Self::new((Self::MODULUS.0 as i64 + reduced) as u64)
        } else {
            Self::new(reduced as u64)
        }
    }
}

impl Into<i32> for Combined {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<f64> for Combined {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

impl Copy for Combined {}

impl Add for Combined {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % Self::MODULUS.0)
    }
}

impl Sub for Combined {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (result, under) = self.0.overflowing_sub(other.0);
        Self(result.wrapping_add(Self::MODULUS.0 * (under as u64)))
    }
}

impl SubAssign for Combined {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Combined {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(((self.0 as u128 * other.0 as u128) % Self::MODULUS.0 as u128) as u64)
    }
}
