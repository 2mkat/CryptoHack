use std::io::{Read, Write};
use std::net::TcpStream;
use regex::Regex;
use num_bigint::{BigUint, ToBigUint, RandBigInt};


fn proof_of_knowledge() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("socket.cryptohack.org:13425")?;
    // let mut stream = TcpStream::connect("localhost:13425")?;

    // read from socket
    let mut buffer = [0; 4096];
    let mut n = stream.read(&mut buffer)?;
    println!("\nServer says: {}", String::from_utf8_lossy(&buffer[..n]));

    
    let w = BigUint::parse_bytes(b"5a0f15a6a725003c3f65238d5f8ae4641f6bf07ebf349705b7f1feda2c2b051475e33f6747f4c8dc13cd63b9dd9f0d0dd87e27307ef262ba68d21a238be00e83", 16).unwrap();
    let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
    let q = BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap();
    let g = 2.to_biguint().unwrap();

    let r = BigUint::parse_bytes(b"5110646754462526066632280957922573803721003103330033431487250946488136096627339164026953858153933693021946070615039644208058415448859970747693890672164154", 10).unwrap();
    let a = BigUint::parse_bytes(b"24019490195618270613604375156213313283256780548049850579405341253641771075513175133276963174786909329635835024061543282778116650185928318117665875212985248", 10).unwrap();


    // Generate r from Zq
    // let mut rng = rand::thread_rng();
    // let two = 2.to_biguint().unwrap();
    // let r = rng.gen_biguint_range(&two, &q);
    // println!("\nGenerate random r = {:?}", r);

    // Calculate a = g^r mod p
    // let a = g.modpow(&r, &p);
    // println!("\nCalculate a = {:?}", a);

    let mut json_string = format!("{{\"a\": {}}}\n", a.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;


    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("Server says: {}", String::from_utf8_lossy(&buffer[..n]));

    let raw_json = String::from_utf8_lossy(&buffer[..n]);
    let re = Regex::new(r#""e"\s*:\s*(\d+)"#).unwrap();

    let e_str = re.captures(&raw_json).unwrap();
    let e = BigUint::parse_bytes((&e_str[1]).as_bytes(), 10).unwrap();

    // if let Some(cap) = re.captures(&raw_json) {
    //     let e_str = &cap[1];
    //     let e = BigUint::parse_bytes(e_str.as_bytes(), 10).unwrap();
    //     println!("e = {}", e);
    // } else {
    //     println!("non");
    // }

    let z = (r + e * w)  % q;
    println!("\nCalculat z = {}", z);

    json_string = format!("{{\"z\": {}}}\n", z.to_str_radix(10));
    stream.write_all(json_string.as_bytes())?;

    buffer = [0; 4096];
    n = stream.read(&mut buffer)?;
    println!("Server says: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}



// anyhow = "*"
// num-bigint = { version = "*", features = ["rand"] }
// rand = "0.8.5"
// regex = "1.10.5"

// use num_bigint::{BigUint, RandBigInt, ToBigUint};
// use regex::Regex;
// use std::io::{BufRead, BufReader, Write};
// use std::net::TcpStream;
// use std::str::FromStr;

// fn send_and_receive(stream: &mut TcpStream, data: &str) -> anyhow::Result<String> {
//     stream.write_all(data.as_bytes())?;

//     let mut reader = BufReader::new(stream);
//     let mut response = String::new();
//     reader.read_line(&mut response)?;

//     Ok(response)
// }

// fn main() -> anyhow::Result<()> {
//     let p = BigUint::parse_bytes(b"1ed344181da88cae8dc37a08feae447ba3da7f788d271953299e5f093df7aaca987c9f653ed7e43bad576cc5d22290f61f32680736be4144642f8bea6f5bf55ef", 16).unwrap();
//     let q = BigUint::parse_bytes(b"f69a20c0ed4465746e1bd047f57223dd1ed3fbc46938ca994cf2f849efbd5654c3e4fb29f6bf21dd6abb662e911487b0f9934039b5f20a23217c5f537adfaaf7", 16).unwrap();
//     let g = 2.to_biguint().unwrap();
//     let w = BigUint::parse_bytes(b"5a0f15a6a725003c3f65238d5f8ae4641f6bf07ebf349705b7f1feda2c2b051475e33f6747f4c8dc13cd63b9dd9f0d0dd87e27307ef262ba68d21a238be00e83", 16).unwrap();

//     let mut rng = rand::thread_rng();
//     let r = rng.gen_biguint_below(&q);
//     let a = g.modpow(&r, &p);

//     let mut stream = TcpStream::connect(("socket.cryptohack.org", 13425))?;

//     let mut reader = BufReader::new(&stream);
//     let mut response = String::new();
//     reader.read_line(&mut response)?;
//     println!("Welcome: {response}");

//     let json_a = format!("{{\"a\": {}}}", a);
//     let response1 = send_and_receive(&mut stream, &json_a)?;
//     println!("Response 1: {}", response1);

//     let re = Regex::new(r#""e":\s*(\d+),"#)?;
//     let e = BigUint::from_str(
//         re.captures(&response1)
//             .and_then(|captures| captures.get(1))
//             .ok_or(anyhow::anyhow!("Cannot get e"))?
//             .as_str(),
//     )?;

//     let z = (r + (e * w) % &q) % &q;
//     let json_z = format!("{{\"z\": {}}}", z);
//     let response2 = send_and_receive(&mut stream, &json_z)?;
//     println!("Response 2: {}", response2);

//     Ok(())
// }