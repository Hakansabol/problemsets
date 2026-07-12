// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
// Functions prefixed with "/// zt..." are snippets and can be read in my dotfiles: https://github.com/Hakansabol/dotfiles/blob/main/nvim/lua/snips/snips_rust.lua
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

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
    if n > 1 {
        // catch prime factors bigger than sqrt(n)
        v.push(n);
    }
    v // return the vector as owned
}
/// zt: Frequency Map
/// Generate a frequency map from a vector
/// vec![1,1,3] becomes {1: 2, 3: 1}
use std::hash::Hash;
fn freqmap<T: Eq + Hash>(v: &Vec<T>) -> std::collections::HashMap<&T, i32> {
    let mut hm = std::collections::HashMap::new();
    for a in v {
        *hm.entry(a).or_insert(0) += 1;
    }
    hm
}

fn solve() {
    let tv = take_vector();
    let (x, y, k) = (tv[0], tv[1], tv[2]);
    let (xf, yf) = (prime_factorization(x), prime_factorization(y));
    let (mut fx, mut yx) = (freqmap(&xf), freqmap(&yf));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
