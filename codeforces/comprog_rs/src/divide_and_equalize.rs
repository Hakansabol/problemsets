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

use std::collections::HashMap;

use template::*;

/// zt: Prime Factorization
/// Returns the prime factors of n.
/// TODO: Optimize to only test prime numbers
fn prime_factorization(n: i64) -> Vec<i64> {
    let mut v = vec![];
    let mut n = n;
    while n & 1 == 0 {
        n /= 2;
        v.push(2);
    }
    let mut div = 3;
    while n > 1 && div * div <= n {
        while n % div == 0 {
            n /= div;
            v.push(div);
        }
        div += 2;
    }
    if n > 1 {
        // catch prime factors bigger than sqrt(n)
        v.push(n);
    }
    v // return the vector as owned
}
fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let mut factormap = HashMap::new();

    for a in v {
        let tv = prime_factorization(a as i64);
        for b in tv {
            *factormap.entry(b).or_insert(0) += 1;
        }
    }

    for a in factormap.values() {
        if a % n != 0 {
            println!("{}", "NO");
            return;
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
