use std::io::{Read, Write};
use std::net::TcpStream;
use regex::Regex;
use num_bigint::{BigUint, RandBigInt, ToBigInt, ToBigUint};

pub fn special_soundness() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("socket.cryptohack.org:13426")?;
    // let mut stream = TcpStream::connect("localhost:13425")?;

    // read from socket
    let mut buffer = [0; 4096];
    let mut n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    let raw_json = String::from_utf8_lossy(&buffer[..n]);
    let re_a = Regex::new(r#""a"\s*:\s*(\d+)"#).unwrap();
    let re_y = Regex::new(r#""y"\s*:\s*(\d+)"#).unwrap();

    let a_str = re_a.captures(&raw_json).unwrap();
    let a = BigUint::parse_bytes((&a_str[1]).as_bytes(), 10).unwrap();
    let y_str = re_y.captures(&raw_json).unwrap();
    let y = BigUint::parse_bytes((&y_str[1]).as_bytes(), 10).unwrap();

    let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
    let q = BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap();
    let g = 2.to_biguint().unwrap();


    // Generate e from Zc > 2^511
    let mut rng = rand::thread_rng();
    let two = 2.to_biguint().unwrap();
    let ubound = two.pow(511);
    let e = rng.gen_biguint_range(&two, &ubound);
    println!("\nGenerate random e = {:?}", e);


    let mut json_string = format!("{{\"e\": {}}}\n", e.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;


    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    let mut raw_json = String::from_utf8_lossy(&buffer[..n]);
    let mut re_z = Regex::new(r#""z"\s*:\s*(\d+)"#).unwrap();

    let mut z_str = re_z.captures(&raw_json).unwrap();
    let z = BigUint::parse_bytes((&z_str[1]).as_bytes(), 10).unwrap();

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));


    // Generate e from 0 < e  2^511
    let e2 = rng.gen_biguint_range(&two, &ubound);
    println!("\nGenerate random e2 = {:?}", e2);


    json_string = format!("{{\"e\": {}}}\n", e2.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;


    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    raw_json = String::from_utf8_lossy(&buffer[..n]);
    re_z = Regex::new(r#""z2"\s*:\s*(\d+)"#).unwrap();

    z_str = re_z.captures(&raw_json).unwrap();
    let z2 = BigUint::parse_bytes((&z_str[1]).as_bytes(), 10).unwrap();

    // Calculate flag
    let diff_e = e + &q - e2;
    let inv_e = (&diff_e).modinv(&q).unwrap();

    let w = ((z + &q - z2) * inv_e) % &q;
    println!("w = {}", w);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn run_task() {
//         let _ = special_soundness();
//     }
// }