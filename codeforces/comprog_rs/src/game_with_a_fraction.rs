// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<u64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let v = take_vector();
    let (p, q) = (v[0], v[1]);

    println!(
        "{}",
        if p >= q {
            "Alice"
        } else if (q - p) * 3 <= q {
            "Bob"
        } else {
            "Alice"
        }
    );
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
// bob needs to:
// drain one number to zero
//
