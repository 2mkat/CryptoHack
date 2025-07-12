use std::io::{Read, Write};
use std::net::TcpStream;
use regex::Regex;
use num_bigint::{BigUint, RandBigInt, ToBigInt, ToBigUint};

pub fn non_interactive() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("socket.cryptohack.org:13428")?;
    
    let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
    let q = BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap();
    let y = BigUint::parse_bytes(b"1a1b551084ac43cc3ae2de2f89c6598a081f220010180e07eb62d0dee9c7502c1401d903018d9d7b06bff2d395c46795aa7cd8765df5ebe7414b072c8289170f0", 16).unwrap();
    let w = BigUint::parse_bytes(b"db968f9220c879b58b71c0b70d54ef73d31b1627868921dfc25f68b0b9495628b5a0ea35a80d6fd4f2f0e452116e125dc5e44508b1aaec89891dddf9a677ddc0", 16).unwrap();
    let g = 2.to_biguint().unwrap();

    // read from socket
    let mut buffer = [0; 4096];
    let mut n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    let mut rng = rand::thread_rng();
    let two = 2.to_biguint().unwrap();
    
    // Generate z from Zp
    // let r =  rng.gen_biguint_range(&two, &q);
    // let a = g.modpow(&r, &p);
    // println!("r = {}", r);
    // println!("a = {}", a);
    let r = BigUint::parse_bytes(b"7011635289130178048138269554323329525468401926677348549399114448200980557023171943002455989032935368050116754247414345368771992349155628773258052335711968", 10).unwrap();
    let a = BigUint::parse_bytes(b"4693971022656757791923473103708574211451316009603685879610838660442103215936828612563009354392247255062984157931999443212261868582111443345818421236388185", 10).unwrap();
    let e = BigUint::parse_bytes(b"298760880453256942074456672274356665841799930794151813023944197348389119614214282671082900559836915055421127896570921192105259557195442695758156332318793", 10).unwrap();
    let z = r + e*w % q;

    let json_string = format!("{{\"a\": {}, \"z\": {}}}\n", a.to_str_radix(10), z.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_task() {
        let _ = non_interactive();
    }
}