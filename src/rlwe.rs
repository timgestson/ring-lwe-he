use super::field::Field;
use super::polynomial::{
    add, div_with_rem, gen_cyclical_poly, gen_gaussian, gen_ternary, gen_uniform, mul, negate,
    scale,
};
use core::convert::*;

pub trait Rlwe {
    // Plaintext Field
    type PF: Field;
    // CipherText Field
    type CF: Field;

    const TERMS: usize;

    fn cyclical() -> Vec<Self::CF> {
        gen_cyclical_poly(Self::TERMS)
    }

    fn gen_keys() -> (Vec<Self::CF>, (Vec<Self::CF>, Vec<Self::CF>)) {
        let sk: Vec<Self::CF> = gen_ternary(Self::TERMS);
        let a: Vec<Self::CF> = gen_uniform(Self::TERMS, Self::CF::MODULUS.into());
        let e: Vec<Self::CF> = gen_gaussian(Self::TERMS, 0., 3.2);
        let pk1: Vec<Self::CF> =
            div_with_rem(&add(&negate(&mul(&a, &sk)), &e), &Self::cyclical()).1;
        let pk2 = a;
        (sk, (pk1, pk2))
    }

    fn encrypt(
        pk1: Vec<Self::CF>,
        pk2: Vec<Self::CF>,
        plaintext: Vec<Self::PF>,
    ) -> (Vec<Self::CF>, Vec<Self::CF>) {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let enc = scale(&plaintext, cf_mod / pf_mod);
        let u: Vec<Self::CF> = gen_ternary(Self::TERMS);
        let e1: Vec<Self::CF> = gen_gaussian(Self::TERMS, 0., 3.2);
        let e2: Vec<Self::CF> = gen_gaussian(Self::TERMS, 0., 3.2);
        let c1: Vec<Self::CF> =
            div_with_rem(&add(&add(&mul(&pk1, &u), &e1), &enc), &Self::cyclical()).1;
        let c2: Vec<Self::CF> = div_with_rem(&add(&mul(&pk2, &u), &e2), &Self::cyclical()).1;
        (c1, c2)
    }
    fn decrypt(sk: Vec<Self::CF>, c1: Vec<Self::CF>, c2: Vec<Self::CF>) -> Vec<Self::PF> {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let noisy_plaintext = div_with_rem(&add(&c1, &mul(&c2, &sk)), &Self::cyclical()).1;
        scale(&noisy_plaintext, pf_mod / cf_mod)
    }
}
