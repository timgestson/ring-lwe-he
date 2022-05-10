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
    let (sk, (pk1, pk2)) = Ring::gen_keys();
    let mut m_vec = vec![Plaintext::ZERO; 16];
    m_vec[0] = Plaintext::new(3);
    let m = Poly::new(m_vec);
    let (c1, c2) = Ring::encrypt(pk1, pk2, m);
    let result = Ring::decrypt(sk, c1.clone() + c1, c2.clone() + c2);
    println!("{:?}", result.0);
}
