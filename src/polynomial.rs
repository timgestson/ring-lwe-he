use super::field::Field;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};

pub fn is_zero<F: Field>(poly: &[F]) -> bool {
    poly.is_empty() || poly.iter().all(|&coef| coef.is_zero())
}

pub fn degree<F: Field>(poly: &[F]) -> usize {
    poly.len() - 1
}

pub fn add<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let result_len = core::cmp::max(a.len(), b.len());
    let mut result = Vec::with_capacity(result_len);
    for i in 0..result_len {
        let c1 = if i < a.len() { a[i] } else { F::ZERO };
        let c2 = if i < b.len() { b[i] } else { F::ZERO };
        result.push(c1 + c2);
    }
    result
}

pub fn mul<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
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

pub fn div_with_rem<F: Field>(a: &[F], b: &[F]) -> (Vec<F>, Vec<F>) {
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

pub fn gen_uniform<F: Field>(size: usize, modulus: i32) -> Vec<F> {
    let mut rng = rand::thread_rng();
    let field = Uniform::from(0..modulus);
    (0..size)
        .map(|_| field.sample(&mut rng) as i32)
        .map(F::from)
        .collect()
}

pub fn gen_gaussian<F: Field>(size: usize, mu: f64, sigma: f64) -> Vec<F> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mu, sigma).unwrap();
    (0..size)
        .map(|_| normal.sample(&mut rng) as i32)
        .map(F::from)
        .collect()
}

pub fn scale<F1: Field, F2: Field>(a: &[F1], factor: f64) -> Vec<F2> {
    a.iter()
        .map(|&f| {
            let float: f64 = f.into();
            F2::new((float * factor).round() as u32)
        })
        .collect()
}

pub fn gen_cyclical_poly<F: Field>(n: usize) -> Vec<F> {
    let mut poly = vec![F::ONE];
    for _ in 0..(n - 1) {
        poly.push(F::ZERO);
    }
    poly.push(F::ONE);
    poly
}

pub fn negate<F: Field>(a: &[F]) -> Vec<F> {
    a.iter().map(|f| f.neg()).collect()
}
