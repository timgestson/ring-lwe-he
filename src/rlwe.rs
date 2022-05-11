use super::field::Field;
use super::polynomial::Poly;
use core::convert::*;

pub trait Rlwe {
    // Plaintext Field
    type PF: Field;
    // Ciphertext Field
    type CF: Field;

    // Base for relinearization (2)
    const BASE: u32 = 2;

    const TERMS: usize;

    fn cyclical() -> Poly<Self::CF> {
        Poly::gen_cyclical(Self::TERMS)
    }

    fn gen_secret_key() -> Poly<Self::CF> {
        let sk: Poly<Self::CF> = Poly::gen_ternary(Self::TERMS);
        sk
    }

    fn gen_public_key(sk: Poly<Self::CF>) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let a: Poly<Self::CF> = Poly::gen_uniform(Self::TERMS);
        let e: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let pk2 = a.clone();
        let pk1: Poly<Self::CF> = (-(a * sk) + e) % Self::cyclical();
        (pk1, pk2)
    }

    fn gen_eval_key(sk: Poly<Self::CF>) -> Vec<(Poly<Self::CF>, Poly<Self::CF>)> {
        let modulus: f64 = Self::CF::MODULUS.into();
        let l = modulus.log(Self::BASE as f64).round();
        (0..l as i64).map(|i| {
            let a: Poly<Self::CF> = Poly::gen_uniform(Self::TERMS);
            let e: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
            (-(a.clone() * sk.clone() + e) + (sk.clone() * sk.clone()).scale((Self::BASE as i64 ^i) as f64), a)
        }).collect()
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

    fn eval_mul(ek: Vec<(Poly<Self::CF>, Poly<Self::CF>)>, c1: Poly<Self::CF>, c2: Poly<Self::CF>, c3: Poly<Self::CF>, c4: Poly<Self::CF>) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        //let l = cf_mod.log(Self::BASE as f64).round();
        let factor = pf_mod / cf_mod;
        let f1: Poly<Self::CF> = (c1.clone() * c3.clone()).scale(factor);
        let f2: Poly<Self::CF> = (c1.clone() * c4.clone() + c2.clone() * c3.clone()).scale(factor);
        let f3: Poly<Self::CF> = (c2.clone() * c4.clone()).scale(factor);
        ek.iter().enumerate().fold((f1.clone(), f2.clone()), |acc, (i, eki)|{
           (acc.0 + eki.0.clone() * f3.clone().pow(i), acc.1 + eki.1.clone() * f3.clone().pow(i))
        }) 
    }
}
