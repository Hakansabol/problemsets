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

/// ztemplate: Sieve of Eratosthenes [first n primes]
/// cnt: the number of primes to find, starting with `1`.
fn eratosthenes(cnt: usize) -> Vec<i64> {
    assert!(cnt <= 1001000, "cnt too large for the sieve (> 1.001M)");
    let mut sieve: Vec<bool> = vec![false; cnt * 16]; // the sieve could be a vector of u8 for memory efficiency.
    let mut basin: Vec<i64> = vec![]; // the basin is the output where the primes fall into.
    let mut point: i64 = 2; // The point is the point that the sieve is currently at.
    while (basin.len() < cnt) {
        // if this item is not flagged
        if !sieve[point as usize] {
            basin.push(point);
            // update the sieve
            let mut sieve_point = point;
            while sieve_point < sieve.len() as i64 {
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
    let n = take_int() as usize;
    let mut v = take_vector();
    v.sort();

    let primes = eratosthenes(n);
    let mut diff: i64 = v.iter().sum::<i64>() - primes.iter().sum::<i64>();
    let mut ans = 0;
    for i in 0..n {
        if diff >= 0 {
            break;
        }
        diff -= v[i];
        diff += primes[primes.len() - 1 - i];
        ans += 1;
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
