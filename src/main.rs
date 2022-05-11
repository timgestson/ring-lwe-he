mod field;
mod fields;
mod polynomial;
mod rlwe;

use crate::field::Field;
use crate::fields::ciphertext::Ciphertext;
use crate::fields::plaintext::Plaintext;
use crate::polynomial::Poly;
use crate::rlwe::Rlwe;

fn main() {
    struct Ring;
    impl Rlwe for Ring {
        type PF = Plaintext;
        type CF = Ciphertext;
        const TERMS: usize = 16;
    }
    let sk = Ring::gen_secret_key();
    let (pk1, pk2) = Ring::gen_public_key(sk.clone());
    let ek = Ring::gen_eval_key(sk.clone());
    let mut m_vec = vec![Plaintext::ZERO; 16];
    m_vec[0] = Plaintext::new(2);
    let m = Poly::new(m_vec);
    let (c1, c2) = Ring::encrypt(pk1.clone(), pk2.clone(), m.clone());
    let (c3, c4) = Ring::encrypt(pk1, pk2, m);
    let product = Ring::eval_mul(ek, c1, c2, c3, c4);
    let result = Ring::decrypt(sk.clone(), product.0, product.1);
    //let result = Ring::decrypt(sk.clone(), c1 + c3, c2+ c4);
    println!("{:?}", result);
}
