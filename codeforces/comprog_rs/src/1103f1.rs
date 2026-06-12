// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
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
    let mut div = 2;
    while n > 1 && div * div <= n {
        while n % div == 0 {
            n /= div;
            v.push(div);
        }
        div = match div {
            2 => 3,
            div => div + 2,
        };

    }
    if n > 1 { // catch prime factors bigger than sqrt(n)
        v.push(n);
    }
    v // return the vector as owned
}

fn solve() {
    let tv = take_vector();
    let (n,_x) = (tv[0] as usize, tv[1]);
    let v = take_vector();
    let mut pv: Vec<Vec<i64>> = v.iter().map(|&x| prime_factorization(x)).collect();
    let mut hm: HashMap<i64,i64> = HashMap::new();

    for a in pv {
        for b in a {
            *hm.entry(b).or_insert(1) += 1;
        }
    }

    println!("{}", hm.iter().fold(1i64, |acc,x| (acc * x.1) % 1000000007));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
