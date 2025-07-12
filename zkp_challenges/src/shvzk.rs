use std::io::{Read, Write};
use std::net::TcpStream;
use regex::Regex;
use num_bigint::{BigUint, RandBigInt, ToBigInt, ToBigUint};

pub fn shvzk() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("socket.cryptohack.org:13427")?;
    // let mut stream = TcpStream::connect("localhost:13427")?;

    // read from socket
    let mut buffer = [0; 4096];
    let mut n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    let raw_json = String::from_utf8_lossy(&buffer[..n]);
    let re_e = Regex::new(r#""e"\s*:\s*(\d+)"#).unwrap();
    let re_y = Regex::new(r#""y"\s*:\s*(\d+)"#).unwrap();

    let e_str = re_e.captures(&raw_json).unwrap();
    let e = BigUint::parse_bytes((&e_str[1]).as_bytes(), 10).unwrap();
    let y_str = re_y.captures(&raw_json).unwrap();
    let y = BigUint::parse_bytes((&y_str[1]).as_bytes(), 10).unwrap();


    let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
    let q = BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap();
    let g = 2.to_biguint().unwrap();


    // Generate a from Zp
    let mut rng = rand::thread_rng();
    let two = 2.to_biguint().unwrap();
    
    // Generate z from Zp
    let mut z = rng.gen_biguint_range(&two, &q);
    let a = g.modpow(&z, &p) * (y.modpow(&e, &p)).modinv(&p).unwrap() % &p;


    let json_string = format!("{{\"a\": {}, \"z\": {}}}\n", a.to_str_radix(10), z.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn run_task() {
//         let _ = shvzk();
//     }
// }