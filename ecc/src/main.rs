use std::{fmt::{self, write, Error}, ops::Rem};
use num_bigint::{BigInt, BigUint};
use sha1::{Sha1, Digest};

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

struct Curve {
    a: BigInt,
    b: BigInt,
    p: BigInt,
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub const ZERO: Self = Self{x: BigInt::ZERO, y: BigInt::ZERO};
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Curve {
    fn add(&self, p: &Point, q: &Point) -> Point {
        // check for belong to curve

        if *p == Point::ZERO {
            return q.clone();
        }
        if *q == Point::ZERO {
            return p.clone();
        }
        if p.x == q.x && (&p.y + &q.y) % &self.p == BigInt::ZERO {
            return Point::ZERO;
        }

        let lambda = if p == q {
            (BigInt::from(3) * p.x.pow(2) + &self.a) * (BigInt::from(2) * &p.y).modinv(&self.p).unwrap()
        } else {
            (&q.y - &p.y) * (&q.x - &p.x).modinv(&self.p).unwrap()
        };

        let x = (lambda.pow(2) - &p.x - &q.x) % &self.p;
        let y = (lambda * (&p.x - &x) - &p.y) % &self.p;

        let res_p =  Point{ 
            x: if x < BigInt::ZERO {&self.p + x } else {x},
            y: if y < BigInt::ZERO {&self.p + y } else {y}
        };

        assert_eq!(self.check_point_in_curve(&res_p), true);
        return res_p;
    }

    fn neg(&self, p: &Point) -> Point {
        Point{x: p.x.clone(), y:  (-&p.y) % &self.p}
    }

    fn check_point_in_curve(&self, p: &Point) -> bool {
        if (p.y.pow(2) % &self.p) == (p.x.pow(3) + &self.a * &p.x + &self.b) % &self.p {
            return true;
        } else {
            return false;
        }
    }

    fn scalar_mul(&self, p: &Point, n: u64) -> Point {
        let mut scalar = n;
        let mut q = p.clone();
        let mut r = Point::ZERO;

        while scalar > 0 {
            if scalar % 2 == 1 {
                r = self.add(&r, &q);
            }  
            q = self.add(&q, &q);
            scalar /= 2;
    
        }
        assert_eq!(self.check_point_in_curve(&r), true);
        return r;
    }
}

impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Y^2 = X^3 + {}X + {} mod {}", self.a, self.b, self.p)
    }
}

fn inverse(module_p: i64, x: i64) -> i64 {
    let (_, _, j) = ex_gcd(module_p, x);

    assert_eq!(1, (j * x).rem_euclid(module_p));
    j
}
fn main() {
    
    println!("{}", "//--------------- TASK 1 - Point Negation --------------- //");

    let curve = Curve{a: BigInt::from(497), b: BigInt::from(1768), p: BigInt::from(9739)};
    println!("P = -Q = {}", curve.neg(&Point {x: BigInt::from(8045), y: BigInt::from(6936)}));

    println!("{}", "//--------------- TASK 2 - Point Addition --------------- //");

    let mut q = Point{x: BigInt::from(1539), y: BigInt::from(4742)};
    let mut p = Point{x: BigInt::from(493), y: BigInt::from(5564)};
    let mut r = Point{x: BigInt::from(4403), y: BigInt::from(5202)};
    println!("S(x, y) = {}", curve.add(&p, &curve.add(&p, &curve.add(&q, &r))));

    let mut x = Point{x: BigInt::from(5274), y: BigInt::from(2841)};
    let mut y = Point{x: BigInt::from(8669), y: BigInt::from(740)};
    println!("X + Y = {}", curve.add(&x, &y));
    println!("X + X = {}", curve.add(&x, &x));

    println!("{}", "//--------------- TASK 3 - Scalar Multiplication ------- //");

    x = Point{x: BigInt::from(5323), y: BigInt::from(5438)};
    println!("[1337] * X = {}", curve.scalar_mul(&x, 1337));

    p = Point{x: BigInt::from(2339), y: BigInt::from(2213)};
    println!("[7863] * P = {}", curve.scalar_mul(&p, 7863));

    println!("{}", "//--------------- TASK 5 - Curves and Logs ------------ //");
    let q_alice = Point{x: BigInt::from(815), y: BigInt::from(3190)};
    let n_b = 1829;
    let x: BigInt = curve.scalar_mul(&q_alice, n_b).x;
    let mut hash = Sha1::new();
    hash.update( x.to_u64_digits().1[0].to_string());

    println!("S = n_b * Q_A = {}", curve.scalar_mul(&q_alice, n_b));
    println!("x = {:x}", hash.finalize());

    println!("{}", "//-------------- TASK 6 - Efficient Exchange --------- //");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_point1() {
        unimplemented!()
    }
}