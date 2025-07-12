// In this file you can find realization for Schnorr protocol for prove knowledge of 
// a discrete logarithm
// Also it is a solution for https://cryptohack.org/challenges/zkp/ challenge 2

extern crate rand;

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;

struct Group {
    p: BigUint,
    q: BigUint,
    g: u32,
}

// Diffie-Hellman group (512 bits)
// p = 2*q + 1 where p,q are both prime, and 2 modulo p generates a group of order q
fn setup_group() -> Group {
    Group{
        p: BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap(),
        q: BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap(),
        g: 2,
    }
}

// TODO: realize class for verifier part
//fn verifier() {
// }

// TODO: realize class for prover part
fn prover(group: Group) {
    let mut rng = rand::thread_rng();
    let two = 2.to_biguint().unwrap();
    let r = rng.gen_biguint_range(&two, &group.q);
    println!("Generate random r = {:?}", r);

    let g = group.g.to_biguint().unwrap();
    let a = g.modpow(&r, &group.p);
    println!("Calculate a = {:?}", a);

    // try to convert a to bytes
    println!("{:?}", a.to_bytes_be());
}

// TODO:: Schnorr protocol
// steps for interective without P and V


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover() {
        prover(setup_group());

        let t1 = BigUint::parse_bytes(b"10", 10).unwrap();
        let t2 = BigUint::parse_bytes(b"2c", 16).unwrap();

        println!("{}", t1);
        println!("{}", t2*t1);
    }

    #[test]
    fn test_protocol() {
        let r = BigUint::parse_bytes(b"4554690057351059520292163394872706558815312355218178598951244661581809023670110314804907276214725682546318027314884530470623916707863561529031302378555271", 10).unwrap();
        let a = BigUint::parse_bytes(b"20432887828647733098824067917918712598048557914685770245748586031965933353924959511101558711129025845598469474879490428625855668575823559764480716916032347", 10).unwrap();
        let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
        let e = BigUint::parse_bytes( b"4940205050079520754925222457760248615830607202319687250592038227266228639149764677828410427393394486196521589565876450033833354941567364759986155485823019", 10).unwrap();

        let w = BigUint::parse_bytes(b"5a0f15a6a725003c3f65238d5f8ae4641f6bf07ebf349705b7f1feda2c2b051475e33f6747f4c8dc13cd63b9dd9f0d0dd87e27307ef262ba68d21a238be00e83", 16).unwrap();
        let y = BigUint::parse_bytes(b"514c8f56336411e75d5fa8c5d30efccb825ada9f5bf3f6eb64b5045bacf6b8969690077c84bea95aab74c24131f900f83adf2bfe59b80c5a0d77e8a9601454e5", 16).unwrap();
        let z = (r + &e * w).modinv(&p).unwrap();
        let g = 2.to_biguint().unwrap();

        assert_eq!(g.modpow(&z, &p), a * y.modpow(&e, &p) % p);
    }

    #[test]
    fn test_with_w_x() {
        let w = BigUint::parse_bytes(b"5a0f15a6a725003c3f65238d5f8ae4641f6bf07ebf349705b7f1feda2c2b051475e33f6747f4c8dc13cd63b9dd9f0d0dd87e27307ef262ba68d21a238be00e83", 16).unwrap();
        let y = BigUint::parse_bytes(b"514c8f56336411e75d5fa8c5d30efccb825ada9f5bf3f6eb64b5045bacf6b8969690077c84bea95aab74c24131f900f83adf2bfe59b80c5a0d77e8a9601454e5", 16).unwrap();
        
    }

}