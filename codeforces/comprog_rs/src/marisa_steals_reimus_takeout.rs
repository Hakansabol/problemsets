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

fn solve() {
    let n = take_int();
    let v = take_vector();

    let mut d = vec![0; 4];
    for a in v {
        d[a as usize] += 1;
    }
    let mut ans = d[0] + d[3];
    let midl = std::cmp::min(d[1], d[2]);
    ans += midl;
    d[1] -= midl;
    d[2] -= midl;
    ans += d[2] / 3;
    ans += d[1] / 3;

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
