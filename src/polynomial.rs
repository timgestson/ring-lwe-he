use super::field::{Field};
use core::ops::{Add, Mul, Rem};


fn is_zero<F: Field>(poly: &[F])-> bool {
    poly.is_empty() || poly.iter().all(|&coef| coef.is_zero())
}

fn degree<F: Field>(poly: &[F]) -> usize {
    poly.len() - 1
}

fn add<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let result_len = core::cmp::max(a.len(), b.len());
    let mut result = Vec::with_capacity(result_len);
    for i in 0..result_len {
        let c1 = if i < a.len() { a[i] } else { F::ZERO };
        let c2 = if i < b.len() {
            b[i]
        } else {
            F::ZERO
        };
        result.push(c1 + c2);
    }
    result
}

fn mul<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let result_len = a.len() + b.len() - 1;
    let mut result = vec![F::ZERO; result_len];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let s = a[i] * b[j];
            result[i + j] = result[i + j] + s;
        }
    }
    result
}

fn div_with_rem<F: Field>(a: &[F], b: &[F]) -> (Vec<F>, Vec<F>) {
    let (a_degree, b_degree) = (degree(&a), degree(&b));
    if is_zero(&a) {
        (vec![F::ZERO], Vec::<F>::new())
    } else if is_zero(&b) {
        panic!("Dividing by zero")
    } else if a_degree < b_degree {
        (vec![F::ZERO], a.to_vec())
    } else {
        let mut quotient = vec![F::ZERO; a_degree - b_degree + 1];
        let mut remainder = a.to_vec();
        let divisor_leading_inv = b.last().unwrap().inv();
        while !is_zero(&remainder) && degree(&remainder) >= b_degree {
            let cur_q_coef = *remainder.last().unwrap() * divisor_leading_inv;
            let cur_q_degree = degree(&remainder) - b_degree;
            quotient[cur_q_degree] = cur_q_coef;

            for (i, &div_coef) in b.iter().enumerate() {
                remainder[cur_q_degree + i] -=  cur_q_coef * div_coef;
            }
            
            while let Some(true) = remainder.last().map(|c| c.is_zero()) {
                remainder.pop();
            }
        }
        (quotient, remainder)
    }
}

/*
#[derive(Clone)]
struct Polynomial<F: Field>(Vec<F>);

impl<F: Field> Polynomial<F> {

    fn zero() -> Self {
        Self::new(&[F::ZERO])
    }

    fn new(coef: &[F]) -> Self {
        Polynomial(coef.to_vec())
    }

    fn to_vec(self) -> Vec<F> {
        self.0
    }

    fn degree(self) -> usize {
        self.0.len() - 1
    }

    fn is_zero(self) -> bool {
        self.0.is_empty() || self.0.iter().all(|&coef| coef.is_zero())
    }
}

impl<F: Field> Add for Polynomial<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let result_len = core::cmp::max(self.0.len(), other.0.len());
        let mut result = Vec::with_capacity(result_len);
        for i in 0..result_len {
            let c1 = if i < self.0.len() { self.0[i] } else { F::ZERO };
            let c2 = if i < other.0.len() {
                other.0[i]
            } else {
                F::ZERO
            };
            result.push(c1 + c2);
        }
        Polynomial(result)
    }
}

impl<F: Field> Mul for Polynomial<F> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let result_len = self.0.len() + other.0.len() - 1;
        let mut result = vec![F::ZERO; result_len];
        for i in 0..self.0.len() {
            for j in 0..other.0.len() {
                let s = self.0[i] * other.0[j];
                result[i + j] = result[i + j] + s;
            }
        }
        Polynomial(result)
    }
}

fn gen_normal<F: Field>(size: usize) -> &[F] {

}

fn gen_binary<F: Field>(size: usize) -> &[F] {

}

fn gen_uniform<F: Field>(size: usize) -> &[F] {

}*/

#[test]
fn test() {

    use core::ops::{Add, Mul, Rem, Sub, SubAssign};

    #[derive(Clone, Debug, PartialEq)]
    pub struct F7(pub u32);

    impl F7 {
        pub fn new(num: u32) -> Self {
            Self(num % Self::MODULUS.0)
        }
    }

    impl Copy for F7 {}

    impl Add for F7 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self((self.0 + other.0) % Self::MODULUS.0)
        }
    }

    impl Sub for F7 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            let (result, under) = self.0.overflowing_sub(other.0);
            Self(result.wrapping_add(Self::MODULUS.0 * (under as u32)))
        }
    }

    impl SubAssign for F7 {
        fn sub_assign(self: &mut Self, other: Self) {
            *self = *self - other;
        }
    }

    impl Mul for F7 {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self(((self.0 as u64 * other.0 as u64) % Self::MODULUS.0 as u64) as u32)
        }
    }

    impl Field for F7 {
        const MODULUS: Self = Self(13);
        const ZERO: Self = Self(0);
        const ONE: Self = Self(1);

        fn is_zero(self) -> bool {
            self == Self::ZERO
        }
        
        fn inv(self) -> Self {
            let mut inverse = F7::ZERO;
            for i in 0..Self::MODULUS.0 {
                inverse = F7::new(i);
                if self * inverse == Self::ONE {
                    break
                }
            }
            inverse
        }
        
        fn neg(self) -> Self {
            Self::MODULUS - self
        }
    }

    let n = 4;
    let A = [F7::new(4), F7::new(1), F7::new(11), F7::new(10)];
    let sA = [F7::new(6), F7::new(9), F7::new(11), F7::new(11)];
    let eA = [F7::new(0), F7::new(1).neg(), F7::new(1), F7::new(1)];
    let xN_1 =[F7::new(1), F7::new(0), F7::new(0), F7::new(0), F7::new(1)];
    let enc = div_with_rem(&add(&mul(&div_with_rem(&A, &xN_1).1, &sA), &eA), &xN_1).1;
    println!("{:?}", enc);
}
