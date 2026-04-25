// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i32>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn isnd(v: &Vec<i32>) -> bool {
    let mut m = 0;
    for &a in v {
        if a >= m {
            m = a;
        } else {
            return false;
        }
    }
    true
}

/// ztemplate: Sieve of Eratosthenes [first n primes]
/// cnt: the number of primes to find, starting with `1`.
fn eratosthenes(cnt: usize) -> Vec<u64> {
    assert!(cnt <= 1001000, "cnt too large for the sieve (> 1.001M)");
    let mut sieve: Vec<bool> = vec![false; cnt * 16]; // the sieve could be a vector of u8 for memory efficiency.
    let mut basin: Vec<u64> = vec![]; // the basin is the output where the primes fall into.
    let mut point: u64 = 2; // The point is the point that the sieve is currently at.
    while basin.len() < cnt {
        // if this item is not flagged
        if !sieve[point as usize] {
            basin.push(point);
            // update the sieve
            let mut sieve_point = point;
            while sieve_point < sieve.len() as u64 {
                sieve[sieve_point as usize] = true;
                sieve_point += point;
            }
        }
        // next number
        point += 1;
    }
    basin
}

fn solve() {
    let _n = take_int();
    let v = take_vector();

    if isnd(&v) {
        println!("{}", "Bob");
        return;
    }

    let mut outv = vec![];
    let primes = eratosthenes(2000);

    for a in v {
        let mut a: u64 = a.clone() as u64;
        let mut unprimes = 0;
        for p in &primes {
            unprimes += if a % p == 0 { 1 } else { 0 };
            while a % *p == 0 && a != *p {
                a /= p;
            }
        }
        if unprimes > 1 {
            println!("{}", "Alice");
            return;
        }
        outv.push(a as i32);
    }

    let ans = if isnd(&outv) { "Bob" } else { "Alice" };
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
// y,z must be >1, so primes are safe
