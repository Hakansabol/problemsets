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
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let v = take_vector();
    let (a, b, n, m) = (v[0], v[1], v[2], v[3]);

    let pera = v[2] % (v[0] * (v[2] / (v[0] - 1)));

    let ans = if n % a > 0 && m % b > 0 && n % (n % a) > 0 && m % (m % b) > 0 {
        "YES"
    } else {
        "NO"
    };
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
