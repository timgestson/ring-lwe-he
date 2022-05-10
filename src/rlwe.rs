use super::field::Field;
use super::polynomial::Poly;
use core::convert::*;

pub trait Rlwe {
    // Plaintext Field
    type PF: Field;
    // Ciphertext Field
    type CF: Field;

    const TERMS: usize;

    fn cyclical() -> Poly<Self::CF> {
        Poly::gen_cyclical(Self::TERMS)
    }

    fn gen_keys() -> (Poly<Self::CF>, (Poly<Self::CF>, Poly<Self::CF>)) {
        let sk: Poly<Self::CF> = Poly::gen_ternary(Self::TERMS);
        let a: Poly<Self::CF> = Poly::gen_uniform(Self::TERMS);
        let e: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let pk2 = a.clone();
        let pk1: Poly<Self::CF> = (-(a * sk.clone()) + e) % Self::cyclical();
        (sk, (pk1, pk2))
    }

    fn encrypt(
        pk1: Poly<Self::CF>,
        pk2: Poly<Self::CF>,
        plaintext: Poly<Self::PF>,
    ) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let enc: Poly<Self::CF> = plaintext.scale(cf_mod / pf_mod);
        let u: Poly<Self::CF> = Poly::gen_ternary(Self::TERMS);
        let e1: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let e2: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let c1: Poly<Self::CF> = (pk1 * u.clone() + e1 + enc) % Self::cyclical();
        let c2: Poly<Self::CF> = (pk2 * u + e2) % Self::cyclical();
        (c1, c2)
    }

    fn decrypt(sk: Poly<Self::CF>, c1: Poly<Self::CF>, c2: Poly<Self::CF>) -> Poly<Self::PF> {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let noisy_plaintext = (c2 * sk + c1) % Self::cyclical();
        noisy_plaintext.scale(pf_mod / cf_mod)
    }
}
