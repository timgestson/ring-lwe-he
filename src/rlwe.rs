use crate::field::Field;
use super::f7::{Plaintext, Coefficient};
use super::polynomial::{gen_cyclical_poly, gen_gaussian, gen_uniform, gen_ternary, mul, add, div_with_rem, convert, negate};


/*
trait Rlwe {
    // Degree of the divisor (power of 2)
    const N: u32;
    // Plaintext Modulus
    type T_FIELD: Field;
    // Cipher Modulus
    const Q_FIELD: Field;

}*/


#[test]
fn errors(){
    let e2: Vec<Coefficient> = gen_gaussian(4, 0., 3.2, 19);
    //println!("{:?}", e2);
}

#[test]
fn rlwe(){
    let d = 16;
    let t = 7.;
    let q = 1024.;
    let delta_up = ((1024. / 7.) as u64) as f64;
    let delta_down = 7. / 1024.;
    let cyclical = gen_cyclical_poly(d);
    let sk: Vec<Coefficient> = gen_ternary(d);
    let a: Vec<Coefficient> = gen_uniform(d, Coefficient::MODULUS.0 as i32);
    let e: Vec<Coefficient> = gen_gaussian(d, 0., 3.2, 19);
    let mut m =vec![Plaintext::ZERO; 16];
    m[0] = Plaintext::ONE;
    let m_delta = convert(&m, delta_up);
    
    let pk1: Vec<Coefficient> = (div_with_rem(&add(&negate(&mul(&a, &sk)), &e), &cyclical).1);//.iter().map(|f| f.inv()).collect();
    let pk2 = a.clone();
    let u: Vec<Coefficient> = gen_ternary(d);
    println!("EEEEEEEEEE {:?}", e);
    println!("E      {:?}", div_with_rem(&add(&pk1, &mul(&sk, &pk2)), &cyclical).1);
    //let u: Vec<Coefficient> = gen_gaussian(4, 0., 3.2, 19);
    // assert mod(pk_0 + s * pk_1, c_q, p_q) == e

    let e1: Vec<Coefficient> = gen_gaussian(d, 0., 3.2, 19);
    let e2: Vec<Coefficient> = gen_gaussian(d, 0., 3.2, 19);
    let c1: Vec<Coefficient> = div_with_rem(&add(&add(&mul(&pk1, &u), &e1), &m_delta), &cyclical).1;
    let c2: Vec<Coefficient> = div_with_rem(&add(&mul(&pk2, &u), &e2), &cyclical).1;

    let c3 = add(&c1, &c1);
    let c4 = add(&c2, &c2);
    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", sk);
    println!("FDSFSDFSD {}", (((1024. / 7.) as u64) as f64) * 7. + (1024. % 7.));

    //let m_unecrypted = div_with_rem(&convert_down(&div_with_rem(&add(&c1, &mul(&c2, &sk)), &cyclical).1, F32::new(2)), &xN_2).1;
    let noisy_plaintext = div_with_rem(&add(&c3, &mul(&c4, &sk)), &cyclical).1;
    let unenc: Vec<Plaintext>  = convert(&noisy_plaintext, 7./1024.);
    println!("{:?}", unenc);
}