mod field;
mod fields;
mod polynomial;
mod rlwe;

use crate::field::Field;
use crate::fields::ciphertext::Ciphertext;
use crate::fields::plaintext::Plaintext;
use crate::polynomial::add;
use crate::rlwe::Rlwe;

fn main() {
    struct Ring;
    impl Rlwe for Ring {
        type PF = Plaintext;
        type CF = Ciphertext;
        const TERMS: usize = 16;
    }
    let (sk, (pk1, pk2)) = Ring::gen_keys();
    let mut m = vec![Plaintext::ZERO; 16];
    m[0] = Plaintext::new(3);
    let (c1, c2) = Ring::encrypt(pk1, pk2, m);
    let c3 = add(&c1, &c1);
    let c4 = add(&c2, &c2);
    let result = Ring::decrypt(sk, c3, c4);
    println!("{:?}", result);
}
