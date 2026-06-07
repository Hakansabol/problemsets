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

use template::*;

fn solve() {
    let n = take_int() as usize;
    let mut bv = vec![];
    for _ in 0..n {
        let v = take_vector();
        bv.push(v);
    }
    let weight = bv[0][0];
    let maxe = bv[0][1];
    bv = bv
        .into_iter()
        .filter(|x| x[0] >= weight && x[1] >= maxe)
        .collect();
    println!("{}", if bv.len() > 1 { -1 } else { weight });
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
