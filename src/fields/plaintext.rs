use crate::Field;
use core::fmt::Debug;
use core::ops::{Add, Mul, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Plaintext(pub u32);

impl From<i32> for Plaintext {
    fn from(int: i32) -> Self {
        let reduced = int % Self::MODULUS.0 as i32;
        if reduced < 0 {
            Self::new((Self::MODULUS.0 as i32 + reduced) as u32)
        } else {
            Self::new(reduced as u32)
        }
    }
}

impl Into<i32> for Plaintext {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<f64> for Plaintext {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

impl Copy for Plaintext {}

impl Add for Plaintext {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % Self::MODULUS.0)
    }
}

impl Sub for Plaintext {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (result, under) = self.0.overflowing_sub(other.0);
        Self(result.wrapping_add(Self::MODULUS.0 * (under as u32)))
    }
}

impl SubAssign for Plaintext {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Plaintext {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(((self.0 as u64 * other.0 as u64) % Self::MODULUS.0 as u64) as u32)
    }
}

impl Field for Plaintext {
    const MODULUS: Self = Self(7);
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn new(num: u32) -> Self {
        Self(num % Self::MODULUS.0)
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn inv(self) -> Self {
        let mut inverse = Plaintext::ZERO;
        for i in 0..Self::MODULUS.0 {
            inverse = Plaintext::new(i);
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

#[derive(Clone, Debug, PartialEq)]
pub struct CipherText(pub u32);

impl From<i32> for CipherText {
    fn from(int: i32) -> Self {
        let reduced = int % Self::MODULUS.0 as i32;
        if reduced < 0 {
            Self::new((Self::MODULUS.0 as i32 + reduced) as u32)
        } else {
            Self::new(reduced as u32)
        }
    }
}

impl Into<i32> for CipherText {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<f64> for CipherText {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

impl Copy for CipherText {}

impl Add for CipherText {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % Self::MODULUS.0)
    }
}

impl Sub for CipherText {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (result, under) = self.0.overflowing_sub(other.0);
        Self(result.wrapping_add(Self::MODULUS.0 * (under as u32)))
    }
}

impl SubAssign for CipherText {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for CipherText {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(((self.0 as u64 * other.0 as u64) % Self::MODULUS.0 as u64) as u32)
    }
}

impl Field for CipherText {
    const MODULUS: Self = Self(1024);
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn new(num: u32) -> Self {
        Self(num % Self::MODULUS.0)
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn inv(self) -> Self {
        let mut inverse = CipherText::ZERO;
        for i in 0..Self::MODULUS.0 {
            inverse = CipherText::new(i);
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