use super::field::Field;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};
use core::ops::{Add, Mul, Sub, Rem, Neg};

#[derive(Debug, Clone)]
pub struct Poly<F: Field>(pub Vec<F>);

impl<F: Field> Poly<F> {
    pub fn new(vec: Vec<F>) -> Self { Self(vec) }
    pub fn gen_cyclical(n: usize) -> Self { Self(gen_cyclical(n))}
    pub fn gen_gaussian(n: usize, mu: f64, sigma: f64) -> Self { Self(gen_gaussian(n, mu, sigma))}
    pub fn gen_uniform(n: usize) -> Self { Self(gen_uniform(n, F::MODULUS.into()))}
    pub fn gen_ternary(n: usize) -> Self { Self(gen_ternary(n))}
    pub fn scale<O: Field>(self, factor: f64) -> Poly<O> { Poly(scale(&self.0, factor)) }
    pub fn pow(self, n: usize) -> Self {
        (1..n).fold(self.clone(), |acc, _|{ 
            acc.clone() * self.clone() 
        })
    }
}


impl<F: Field> Add for Poly<F> {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Poly(add(&self.0, &b.0))
    }
}

impl<F: Field> Rem for Poly<F> {
    type Output = Self;

    fn rem(self, b: Self) -> Self {
        Poly(div_with_rem(&self.0, &b.0).1)
    }
}

impl<F: Field> Mul for Poly<F> {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Poly(mul(&self.0, &b.0))
    }
}

impl<F: Field> Neg for Poly<F> {
    type Output = Self;

    fn neg(self) -> Self {
        Poly(negate(&self.0))
    }
}

pub fn is_zero<F: Field>(poly: &[F]) -> bool {
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
        let c2 = if i < b.len() { b[i] } else { F::ZERO };
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
    let (a_degree, b_degree) = (degree(a), degree(b));
    if is_zero(a) {
        (vec![F::ZERO], Vec::<F>::new())
    } else if is_zero(b) {
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
                remainder[cur_q_degree + i] -= cur_q_coef * div_coef;
            }

            while let Some(true) = remainder.last().map(|c| c.is_zero()) {
                remainder.pop();
            }
        }
        (quotient, remainder)
    }
}

pub fn gen_ternary<F: Field>(size: usize) -> Vec<F> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(-1..2_i32))
        .map(F::from)
        .collect()
}

fn gen_uniform<F: Field>(size: usize, modulus: i32) -> Vec<F> {
    let mut rng = rand::thread_rng();
    let field = Uniform::from(0..modulus);
    (0..size)
        .map(|_| field.sample(&mut rng) as i32)
        .map(F::from)
        .collect()
}

fn gen_gaussian<F: Field>(size: usize, mu: f64, sigma: f64) -> Vec<F> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mu, sigma).unwrap();
    (0..size)
        .map(|_| normal.sample(&mut rng) as i32)
        .map(F::from)
        .collect()
}

fn scale<F1: Field, F2: Field>(a: &[F1], factor: f64) -> Vec<F2> {
    a.iter()
        .map(|&f| {
            let float: f64 = f.into();
            F2::new((float * factor).round() as u32)
        })
        .collect()
}

fn gen_cyclical<F: Field>(n: usize) -> Vec<F> {
    let mut poly = vec![F::ONE];
    for _ in 0..(n - 1) {
        poly.push(F::ZERO);
    }
    poly.push(F::ONE);
    poly
}

fn negate<F: Field>(a: &[F]) -> Vec<F> {
    a.iter().map(|f| f.neg()).collect()
}