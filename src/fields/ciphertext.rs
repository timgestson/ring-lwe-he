use crate::Field;
use core::fmt::Debug;
use core::ops::{Add, Mul, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Ciphertext(pub u32);

impl Field for Ciphertext {
    const MODULUS: Self = Self(4096);
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn new(num: u32) -> Self {
        Self(num % Self::MODULUS.0)
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn inv(self) -> Self {
        let mut inverse = Ciphertext::ZERO;
        for i in 0..Self::MODULUS.0 {
            inverse = Ciphertext::new(i);
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

impl From<i32> for Ciphertext {
    fn from(int: i32) -> Self {
        let reduced = int % Self::MODULUS.0 as i32;
        if reduced < 0 {
            Self::new((Self::MODULUS.0 as i32 + reduced) as u32)
        } else {
            Self::new(reduced as u32)
        }
    }
}

impl Into<i32> for Ciphertext {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<f64> for Ciphertext {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

impl Copy for Ciphertext {}

impl Add for Ciphertext {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % Self::MODULUS.0)
    }
}

impl Sub for Ciphertext {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (result, under) = self.0.overflowing_sub(other.0);
        Self(result.wrapping_add(Self::MODULUS.0 * (under as u32)))
    }
}

impl SubAssign for Ciphertext {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Ciphertext {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(((self.0 as u64 * other.0 as u64) % Self::MODULUS.0 as u64) as u32)
    }
}
