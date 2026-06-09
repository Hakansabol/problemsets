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

fn solve() {
    let v = take_vector();
    let (n, x, y, z) = (v[0], v[1], v[2], v[3]);

    let noai = (n - 1) / (y + x) + 1;
    let yesai = ((n - 1) - (x * z)) / (x + y * 10) + 1 + z;

    println!("{}", std::cmp::min(noai, yesai));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
