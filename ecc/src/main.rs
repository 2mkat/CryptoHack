
fn module(prime: i64, x: i64) -> i64 {
    x % prime
}

fn ex_gcd(m: i64, n: i64) -> (i64, i64, i64) {
    if n == 0 {
        return (m, 1, 0);
    } else {
        let (d, i, j) = ex_gcd(n, m.rem_euclid(n));
        return (d, j, i - j * m.div_euclid(n));
    }
}

// ex: 240x = 1 (mod p)
// modular multiplicative inverse
// TODO:: creat gcd function

// fn inverse(p: i64, a: i64) -> i64{
//     if a == 0 {
//         0
//     } else {
//         let mut r: Vec<i64> = vec![];
//         r.push(p);
//         r.push(a);

//         let mut s: Vec<i64> = vec![];
//         s.push(1);
//         s.push(0);

//         let mut t: Vec<i64> = vec![];
//         t.push(0);
//         t.push(1);

//         let mut q: Vec<i64> = vec![];
//         q.push(0);
//         q.push(0);
        
//         let mut i = 2;

//         while r.last() != Some(&1) {
//             // println!("r[i-2] = {} r[i-1] = {}", r[i-2], r[i-1]);
//             q.push(r[i-2]/r[i-1]);
//             r.push(r[i-2] - q[i] * r[i-1]);
//             s.push(s[i-2] - q[i] * s[i-1]);
//             t.push(t[i-2] - q[i] * t[i-1]);
            
//             println!("r{} = {}", i, r[i-2] - q[i] * r[i-1]);
//             println!("s{} = {}", i, s[i-2] - q[i] * s[i-1]);
//             println!("t{} = {}", i, t[i-2] - q[i] * t[i-1]);
//             println!("q{} = {}", i, q[i]);
//             i += 1;
//         }

//         assert_eq!(1, (*t.last().unwrap() * a).rem_euclid(p));

//         (*t.last().unwrap()).rem_euclid(p) 
//     }
// }

fn inverse(module: i64, x: i64) -> i64 {
    let (d, i, j) = ex_gcd(module, x);

    println!("{}", j);
    assert_eq!(1, (j * x).rem_euclid(module));
    j
}

fn point_additin_ec(p: (i64, i64), q:  (i64, i64)) ->  (i64, i64) {
    let a: i64 = 497;
    let b: i64 = 1768;
    let prime: i64 = 9739;

    // add check for belong to curve

    // if p == q {

    // }
    // if p.0 == q.0 && p.1 == -q.1 {

    // }

    let mut lambda: i64 = 0;

    if p.0 != q.0 || p.1 != q.1 {
        println!("invrese = {}", inverse(prime, q.0 - p.0));
        lambda = ((q.1 - p.1) * inverse(prime, q.0 - p.0)).rem_euclid(prime);
    } else {
        println!("Sss");
        println!("invrese = {}",  inverse(prime, 2 * p.1));
        lambda = ((3 * p.0.pow(2) + a)* inverse(prime, 2 * p.1)).rem_euclid(prime);
    }

    println!("lambda = {}", lambda); //7039

    println!("lambda^2 = {}", lambda.pow(2));   // 5228
    let x3 =(lambda.pow(2) - p.0 - q.0).rem_euclid(prime);
    let y3 = (lambda * (p.0 - x3) - p.1).rem_euclid(prime);

    println!("x3 = {} y3 = {}", x3, y3);

    assert_eq!(y3.pow(2).rem_euclid(prime), (x3.pow(3) + a*x3 + b).rem_euclid(prime));

    (x3, y3)

}

fn main() {

    //--------------- TASK 1 - Point Addition --------------- //

    // println!("{:?}", ex_gcd(35, 3));
    // println!("{:?}", inverse(35, 3));

    // println!("{}", inverse(26, 11));
    // (1539,4742) Q
    // (493,5564)  P
    // (4403,5202) R
    // (2130, 2999) P + P
    // (8592, 2572) Q + R
    point_additin_ec((8592, 2572), (2130, 2999));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_gcd() {

    }
}