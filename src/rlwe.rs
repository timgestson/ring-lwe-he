use super::field::Field;
use super::polynomial::Poly;
use core::convert::*;


const P: f64 = 1048576 as f64;

pub trait Rlwe {
    // Plaintext Field
    type PF: Field;
    // Ciphertext Field
    type CF: Field;
    // Relinearization Field
    type RF: Field;

    const TERMS: usize;

    fn cyclical() -> Poly<Self::CF> {
        Poly::gen_cyclical(Self::TERMS)
    }

    fn gen_secret_key() -> Poly<Self::CF> {
        let sk: Poly<Self::CF> = Poly::gen_ternary(Self::TERMS);
        sk
    }

    fn gen_public_key(sk: &Poly<Self::CF>) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let a: Poly<Self::CF> = Poly::gen_uniform(Self::TERMS);
        let e: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let pk2 = a.clone();
        let pk1: Poly<Self::CF> = (-(&a * sk) + &e) % Self::cyclical();
        (pk1, pk2)
    }

    fn gen_eval_key(sk: &Poly<Self::CF>) -> (Poly<Self::RF>, Poly<Self::RF>) {
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let a: Poly<Self::RF> = Poly::gen_uniform(Self::TERMS);
        println!("{:?}", a);
        let e: Poly<Self::RF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let rk2 = a.clone();
        let sk: Poly<Self::RF> = sk.clone().scale(1.);
        let rk1: Poly<Self::RF> = (-(&a * &sk + &e) + ((&sk * &sk) % Self::cyclical().scale(1.)).scale(P)) % Self::cyclical().scale(1.);
        (rk1, rk2)
    }

    fn encrypt(
        pk1: &Poly<Self::CF>,
        pk2: &Poly<Self::CF>,
        plaintext: &Poly<Self::PF>,
    ) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let enc: Poly<Self::CF> = plaintext.scale(cf_mod / pf_mod);
        let u: Poly<Self::CF> = Poly::gen_ternary(Self::TERMS);
        let e1: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let e2: Poly<Self::CF> = Poly::gen_gaussian(Self::TERMS, 0., 3.2);
        let c1: Poly<Self::CF> = (pk1 * &u) + &e1 + &enc % &Self::cyclical();
        let c2: Poly<Self::CF> = (pk2 * &u + &e2) % Self::cyclical();
        (c1, c2)
    }

    fn decrypt(sk: &Poly<Self::CF>, c1: &Poly<Self::CF>, c2: &Poly<Self::CF>) -> Poly<Self::PF> {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let noisy_plaintext = (c2 * sk + c1) % Self::cyclical();
        noisy_plaintext.scale(pf_mod / cf_mod)
    }

    fn eval_mul(rk: &(Poly<Self::RF>, Poly<Self::RF>), ct10: &Poly<Self::CF>, ct11: &Poly<Self::CF>, ct20: &Poly<Self::CF>, ct21: &Poly<Self::CF>) -> (Poly<Self::CF>, Poly<Self::CF>) {
        let cf_mod: f64 = Self::CF::MODULUS.into();
        let pf_mod: f64 = Self::PF::MODULUS.into();
        let factor = pf_mod / cf_mod;
        
        let c0: Poly<Self::CF> = ((ct10 * ct20) % Self::cyclical()).scale(factor);
        let c1: Poly<Self::CF> = ((ct10 * ct21 + ct11 * ct20) % Self::cyclical()).scale(factor);
        let c2: Poly<Self::CF> = ((ct11 * ct21) % Self::cyclical()).scale(factor);
        
        let c2_scaled: Poly<Self::RF> = c2.scale(1.);
        let c20: Poly<Self::CF> = ((&c2_scaled * &rk.0) % Self::cyclical().scale(1.)).scale(1./P);
        let c21: Poly<Self::CF> = ((&c2_scaled * &rk.1) % Self::cyclical().scale(1.)).scale(1./P);
        ((&c0 + &c20) % Self::cyclical(), (&c1 + &c21) % Self::cyclical())
    }
}
